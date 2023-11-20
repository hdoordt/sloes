use anyhow::{bail, Result};
use hyper::{
    body::Bytes, client::conn::http1::SendRequest, server::conn::http1, service::service_fn,
    Request, Response,
};
use hyper_util::rt::TokioIo;
use std::{net::SocketAddr, sync::Arc};
use tokio::net::{TcpListener, TcpStream};
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

    pub async fn serve(self, addr: SocketAddr) -> Result<()> {
        let proxy = |req: Request<hyper::body::Incoming>| async move {
            info!("Got request: {req:?}");
            let Some(host) = req.uri().host() else {
                error!("Can't forward request without host");
                bail!("poep");
            };
            let port = req.uri().port_u16().unwrap_or(80);

            // Open a TCP connection to the remote host
            let address = format!("{}:{}", host, port);
            let stream = TcpStream::connect(address).await?;

            let io = TokioIo::new(stream);

            // Perform a TCP handshake
            let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;
            tokio::task::spawn(async move {
                if let Err(err) = conn.await {
                    println!("Connection failed: {:?}", err);
                }
            });
            let res = sender.send_request(req).await?;

            info!("Got response: {res:?}");

            Ok(res)
        };
        let proxy = service_fn(proxy);

        let listener = TcpListener::bind(addr).await?;

        loop {
            let (stream, _) = listener.accept().await?;
            let io = TokioIo::new(stream);

            tokio::spawn(async move {
                if let Err(e) = http1::Builder::new().serve_connection(io, proxy).await {
                    error!("Error handling connection: {}", e);
                }
            });
        }
    }
}
