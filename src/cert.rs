use std::{collections::HashMap, path::Path, sync::Arc};

use anyhow::Result;
use rcgen::{
    Certificate as RcgenCertificate, CertificateParams, DistinguishedName, DnType, IsCa, KeyPair,
    KeyUsagePurpose,
};
use rustls::Certificate;

use crate::storage::config::{Config, ConfigStore};

pub struct CertManager {
    root: RcgenCertificate,
    domain_certs: HashMap<url::Host, Certificate>,
}

impl CertManager {
    pub async fn load_from_conf(conf: Arc<ConfigStore>) -> Result<Option<Self>> {
        let Config {
            root_cert_path,
            root_key_path,
            ..
        } = conf.data();
        let cert = tokio::fs::read_to_string(root_cert_path).await?;
        let key = tokio::fs::read_to_string(root_key_path).await?;
        let params = CertificateParams::from_ca_cert_pem(&cert, KeyPair::from_pem(&key)?)?;

        Ok(Some(Self {
            root: RcgenCertificate::from_params(params)?,
            domain_certs: HashMap::new(),
        }))
    }

    // generate root certificate
    pub async fn generate(_conf: Arc<ConfigStore>) -> Result<Self> {
        let mut params = CertificateParams::new(vec![]);
        params.is_ca = IsCa::Ca(rcgen::BasicConstraints::Constrained(0));

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
        tokio::fs::write(root_cert_path, cert).await?;
        tokio::fs::write(root_key_path, key).await?;
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
        assert!(matches!(domain, url::Host::Domain(_)));

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
}
