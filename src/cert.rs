use std::{collections::HashMap, fmt, sync::Arc};

use anyhow::Result;
use rcgen::{Certificate as RcgenCertificate, CertificateParams, DnType, IsCa, KeyPair};
use rustls::{server::ResolvesServerCert, Certificate, ServerConfig};
use tokio::fs;
use tracing::error;

use crate::storage::config::{Config, ConfigStore};

#[non_exhaustive]
pub struct CertManager {
    pub root: RcgenCertificate,
    pub domain_certs: HashMap<url::Host, Certificate>,
}

impl fmt::Debug for CertManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CertManager")
            .field("root", &"<...>")
            .field("domain_certs", &"<Certs>")
            .finish()
    }
}

impl CertManager {
    pub async fn load_from_conf(conf: Arc<ConfigStore>) -> Result<Option<Self>> {
        let Config {
            root_cert_path,
            root_key_path,
            ..
        } = conf.data();

        let (cert_exists, key_exists) = futures::join!(
            fs::try_exists(root_cert_path),
            fs::try_exists(root_key_path)
        );
        if !cert_exists? || !key_exists? {
            return Ok(None);
        }

        let cert = fs::read_to_string(root_cert_path).await?;
        let key = fs::read_to_string(root_key_path).await?;
        let params = CertificateParams::from_ca_cert_pem(&cert, KeyPair::from_pem(&key)?)?;

        Ok(Some(Self {
            root: RcgenCertificate::from_params(params)?,
            domain_certs: HashMap::new(),
        }))
    }

    // generate root certificate
    pub async fn generate(_conf: Arc<ConfigStore>) -> Result<Self> {
        let mut params = CertificateParams::new(vec![]);
        params.is_ca = IsCa::Ca(rcgen::BasicConstraints::Unconstrained);

        Ok(Self {
            root: RcgenCertificate::from_params(params)?,
            domain_certs: HashMap::new(),
        })
    }

    pub async fn load_or_generate(conf: Arc<ConfigStore>) -> Result<Self> {
        if let Some(cert_man) = Self::load_from_conf(conf.clone()).await? {
            return Ok(cert_man);
        }

        let cert_man = Self::generate(conf.clone()).await?;
        cert_man.store(conf).await?;
        Ok(cert_man)
    }

    pub async fn store(&self, conf: Arc<ConfigStore>) -> Result<()> {
        let cert = self.root.serialize_pem()?;
        let key = self.root.serialize_private_key_pem();
        let Config {
            root_cert_path,
            root_key_path,
            ..
        } = conf.data();
        fs::write(root_cert_path, cert).await?;
        fs::write(root_key_path, key).await?;
        Ok(())
    }

    pub fn generate_for_domain(&mut self, domain: &url::Host) -> Result<Certificate> {
        if !Self::is_domain(domain) {
            return Err(crate::error::Error::NotADomain)?;
        }

        Ok(self.do_generate_for_domain(domain)?)
    }

    pub fn get_or_generate_for_domain(&mut self, domain: &url::Host) -> Result<&Certificate> {
        if !Self::is_domain(domain) {
            return Err(crate::error::Error::NotADomain)?;
        }

        if !self.domain_certs.contains_key(domain) {
            let cert = self.do_generate_for_domain(domain)?;
            self.domain_certs.insert(domain.clone(), cert);
        }

        Ok(self.domain_certs.get(domain).unwrap())
    }

    fn do_generate_for_domain(&self, domain: &url::Host) -> Result<Certificate> {
        assert!(Self::is_domain(domain));

        let mut params = CertificateParams::new(vec![]);
        params.is_ca = IsCa::NoCa;
        params
            .distinguished_name
            .push(DnType::CommonName, domain.to_string());

        let unsigned = RcgenCertificate::from_params(params)?;

        Ok(Certificate(unsigned.serialize_der_with_signer(&self.root)?))
    }

    fn is_domain(domain: &url::Host) -> bool {
        matches!(domain, url::Host::Domain(_))
    }

    pub fn server_config(self: Arc<Self>) -> ServerConfig {
        ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_cert_resolver(self)
    }
}

impl ResolvesServerCert for CertManager {
    fn resolve(
        &self,
        client_hello: rustls::server::ClientHello,
    ) -> Option<Arc<rustls::sign::CertifiedKey>> {
        let server_name = client_hello.server_name();
        error!("{server_name:?}");
        None
    }
}
