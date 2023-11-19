use anyhow::Result;
use futures::Stream;
use hyper::{
    server::conn::{http1, http2},
    service::service_fn,
    Request, Response,
};
use hyper_util::rt::{TokioExecutor, TokioIo};
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;

use crate::{cert::CertManager, storage::config::ConfigStore, storage::project::Project};

pub struct Proxy {
    config: Arc<ConfigStore>,
    project: Project,
    cert_man: CertManager,
}

impl Proxy {
    pub fn new(config: Arc<ConfigStore>, project: Project, cert_man: CertManager) -> Self {
        Self {
            config,
            project,
            cert_man,
        }
    }

    pub async fn serve(self, addr: SocketAddr) -> Result<()> {
        async fn proxy(req: Request<Vec<u8>>) -> Response<Vec<u8>> {
            todo!()
        }
        let proxy = service_fn(proxy);

        let listener = TcpListener::bind(addr).await?;

        loop {
            let stream = listener.accept().await?;
            let io = TokioIo::new(stream);

            tokio::spawn(async move {
                
            });
        }
    }
}
