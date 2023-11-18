use crate::{config::Config, storage::Storage, cert::CertManager};

pub struct Proxy<'c> {
    config: &'c Config,
    storage: Storage,
    cert_man: CertManager,
}

impl<'c> Proxy<'c> {
    pub fn new(config: &'c Config, storage: Storage, cert_man: CertManager) -> Self {
        Self { config, storage, cert_man }
    }

    pub async fn serve(self) -> ! {
        todo!("Serve a proxy using hyper. Use cert manager to generate certs on demand. USe storage to store request and response data")
    }
}
