use std::{collections::HashMap, fmt, path::Path, sync::Arc};

use anyhow::Result;
use rcgen::{Certificate, CertificateParams, DistinguishedName, IsCa, KeyUsagePurpose};

use crate::storage::config::{Config, ConfigStore};

pub struct CertManager {
    root: Certificate,
    domain_certs: HashMap<String, Certificate>,
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
        todo!()
    }

    pub async fn generate(_conf: Arc<ConfigStore>) -> Result<Self> {
        let mut params = CertificateParams::new(vec![]);
        params.is_ca = IsCa::Ca(rcgen::BasicConstraints::Unconstrained);
        params.key_usages = vec![
            KeyUsagePurpose::DigitalSignature,
            KeyUsagePurpose::DataEncipherment,
            KeyUsagePurpose::KeyCertSign,
        ];

        Ok(Self {
            root: Certificate::from_params(params)?,
            domain_certs: HashMap::new(),
        })
    }

    pub async fn load_or_generate(conf: Arc<ConfigStore>) -> Result<Self> {
        if let Some(cert_man) = Self::load_from_conf(conf.clone()).await? {
            return Ok(cert_man);
        }

        Self::generate(conf).await
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

    pub fn generate_for_domain(&mut self, domain: &str) -> Result<Certificate> {
        if !Self::is_domain(domain) {
            return Err(crate::error::Error::NotADomain)?;
        }

        Ok(Self::do_generate_for_domain(domain))
    }

    pub fn get_or_generate_for_domain(&mut self, domain: &str) -> Result<&Certificate> {
        if !Self::is_domain(domain) {
            return Err(crate::error::Error::NotADomain)?;
        }
        todo!()
        // Ok(self
        //     .domain_certs
        //     .entry(domain.to_owned())
        //     .or_insert_with(|| Self::do_generate_for_domain(domain)))
    }

    fn do_generate_for_domain(domain: &str) -> Certificate {
        assert!(Self::is_domain(domain));
        todo!()
    }

    fn is_domain(domain: &str) -> bool {
        matches!(url::Host::parse(domain), Ok(url::Host::Domain(_)))
    }
}
