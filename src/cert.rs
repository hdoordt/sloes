use std::{collections::HashMap, path::Path, sync::Arc};

use anyhow::Result;

use crate::storage::config::{Config, ConfigStore};

pub struct Certificate {
    // fields
}

pub struct CertManager {
    root: Certificate,
    domain_certs: HashMap<String, Certificate>,
}

impl CertManager {
    pub async fn load_from_conf(conf: Arc<ConfigStore>) -> Result<Option<Self>> {
        todo!()
    }

    pub async fn generate(conf: Arc<ConfigStore>) -> Result<Self> {
        todo!()
    }

    pub async fn load_or_generate(conf: Arc<ConfigStore>) -> Result<Self> {
        if let Some(cert_man) = Self::load_from_conf(conf.clone()).await? {
            return Ok(cert_man);
        }

        Self::generate(conf).await
    }

    pub async fn store(&self, path: impl AsRef<Path>) -> Result<()> {
        todo!()
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
