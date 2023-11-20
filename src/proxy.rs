use anyhow::{bail, Result};
use futures::{Future, TryFutureExt, FutureExt};
use hyper::{
    body::Bytes, client::conn::http1::SendRequest, server::conn::http1, service::service_fn,
    Request, Response,
};
use hyper_util::rt::TokioIo;
use std::{net::SocketAddr, sync::Arc};
use tokio::{net::{TcpListener, TcpStream}, task::{JoinHandle, JoinError}};
use tracing::{error, info};

use http_body_util::Full;

use crate::{
    cert::CertManager,
    storage::project::Project,
    storage::{config::ConfigStore, project::ProjectStore},
};

pub struct Proxy {
    config: Arc<ConfigStore>,
    project: Arc<ProjectStore>,
    cert_man: CertManager,
}

impl Proxy {
    pub fn new(
        config: Arc<ConfigStore>,
        project: Arc<ProjectStore>,
        cert_man: CertManager,
    ) -> Self {
        Self {
            config,
            project,
            cert_man,
        }
    }

    pub async fn serve_https(&mut self, addr: SocketAddr) -> Result<()> {
        let proxy = |req: Request<hyper::body::Incoming>| async move { Ok::<_, anyhow::Error>(()) };

        Ok(())
    }

    pub async fn serve_http(&mut self, addr: SocketAddr) -> Result<impl Future<Output = Result<Result<()>, JoinError>>> {
        let proxy = |req: Request<hyper::body::Incoming>| async move {
            info!("Got request: {req:?}. URI: {:?}", req.uri());

            let host = req.uri().host().unwrap_or("127.0.0.1");
            let port = req.uri().port_u16().unwrap_or(80);

            // Open a TCP connection to the remote host
            let address = format!("{}:{}", host, port);
            let stream = TcpStream::connect(address).await?;

            let io = TokioIo::new(stream);

            // Perform a TCP handshake
            let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;
            tokio::task::spawn(async move {
                if let Err(err) = conn.await {
                    bail!("Connection failed: {:?}", err);
                };
                Ok(())
            });
            let res = sender.send_request(req).await?;

            info!("Got response: {res:?}");

            Ok::<_, anyhow::Error>(res)
        };
        let proxy = service_fn(proxy);

        let listener = TcpListener::bind(addr).await?;
        let task: JoinHandle<Result<()>> = tokio::spawn(async move {
            loop {
                let (stream, _) = listener.accept().await?;
                let io = TokioIo::new(stream);

                tokio::spawn(async move {
                    if let Err(e) = http1::Builder::new().serve_connection(io, proxy).await {
                        error!("Error handling connection: {}", e);
                    }
                });
            }
            
        });
        
        Ok(task)
    }
}
