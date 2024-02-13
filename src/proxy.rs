use anyhow::{bail, Result};
use futures::{Future, FutureExt, TryFutureExt};
use http::uri::Scheme;
use hyper::{
    body::{self, Bytes},
    client::{self, conn::http1::SendRequest},
    server::conn::{http1, http2},
    service::service_fn,
    Method, Request, Response,
};
use hyper_rustls::{ConfigBuilderExt, TlsAcceptor};
use hyper_util::{
    client::legacy::Client,
    rt::{TokioExecutor, TokioIo},
};
use rustls::{server::ResolvesServerCert, ServerConfig};
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio::{
    net::{TcpListener, TcpStream},
    task::{JoinError, JoinHandle},
};
use tracing::{error, info, instrument, span};

use http_body_util::Full;

use crate::{
    cert::CertManager,
    storage::project::Project,
    storage::{config::ConfigStore, project::ProjectStore},
};

#[derive(Debug)]
pub struct Proxy {
    config: Arc<ConfigStore>,
    project: Arc<ProjectStore>,
    cert_man: Arc<CertManager>,
}

impl Proxy {
    pub fn new(
        config: Arc<ConfigStore>,
        project: Arc<ProjectStore>,
        cert_man: Arc<CertManager>,
    ) -> Self {
        Self {
            config,
            project,
            cert_man,
        }
    }

    #[instrument]
    pub async fn serve_https(
        &mut self,
        addr: SocketAddr,
    ) -> Result<impl Future<Output = Result<()>>> {
        let client_tls = rustls::ClientConfig::builder()
            .with_safe_defaults()
            .with_native_roots()?
            .with_no_client_auth();
        let client_https = hyper_rustls::HttpsConnectorBuilder::new()
            .with_tls_config(client_tls)
            .https_only()
            .enable_http1()
            .build();

        /// Uhh not great but who cares tho
        let client: &'static _ = Box::leak(Box::new(
            Client::builder(TokioExecutor::new()).build(client_https),
        ));
        let proxy = {
            move |req: Request<body::Incoming>| async move {
                panic!();
                info!("Got request: {req:?}. URI: {:?}", req.uri());
                if req.method() == Method::CONNECT {
                    match hyper::upgrade::on(req).await {
                        Ok(upgraded) => todo!(),
                        Err(e) => bail!(e),
                    }
                } else {
                    let res = client.request(req).await?;

                    info!("Got response: {res:?}");
                    Ok::<_, anyhow::Error>(res)
                }
            }
        };

        let listener = TcpListener::bind(addr).await?;

        let mut acceptor = TlsAcceptor::builder()
            .with_tls_config(CertManager::server_config(self.cert_man.clone()))
            .with_http11_alpn()
            .with_incoming(listener);

        let proxy = service_fn(proxy);

        let task: JoinHandle<Result<()>> = tokio::spawn(async move {
            loop {
                let (stream, remote) = acceptor.accept().await?;
                tokio::spawn(async move {
                    info!("Connection from {remote}!");
                    match http1::Builder::new().serve_connection(stream, proxy).await {
                        Err(e) => error!("Error handling connection: {}", e),

                        Ok(()) => info!("Done!"),
                    }
                });
            }
        });
        Ok(task.map(|r: Result<Result<()>, JoinError>| match r {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e.into()),
            Err(e) => Err(e.into()),
        }))
    }

    #[instrument]
    pub async fn serve_http(
        &mut self,
        addr: SocketAddr,
    ) -> Result<impl Future<Output = Result<()>>> {
        let proxy = |req: Request<body::Incoming>| async move {
            info!(
                "Got request: {req:?}. URI: {:?}, method: {:?}",
                req.uri(),
                req.method()
            );

            if Some(&Scheme::HTTPS) == req.uri().scheme() {
                
                match hyper::upgrade::on(req).await {
                    Ok(upgraded) => {
                        info!("upgreet: {upgraded:?}");
                        tokio::time::sleep(Duration::from_secs(1)).await;
                        panic!("sdflsf");
                    }
                    Err(e) => {
                        error!("Error upgrading: {e}");
                        tokio::time::sleep(Duration::from_secs(1)).await;
                        bail!("poep")
                    }
                }
            }

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
                        error!("Error handling connection: {:?}", e);
                    }
                    // if let Err(e) = http2::Builder::new(TokioExecutor::new())
                    //     .serve_connection(io, proxy)
                    //     .await
                    // {
                    //     error!("Error handling connection: {}", e);
                    // }
                });
            }
        });

        Ok(task.map(|r: Result<Result<()>, JoinError>| match r {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e.into()),
            Err(e) => Err(e.into()),
        }))
    }

    fn proxy() -
}
