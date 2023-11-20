use anyhow::{bail, Result};
use futures::{Future, FutureExt, TryFutureExt};
use hyper::{
    body::{self, Bytes},
    client::{self, conn::http1::SendRequest},
    server::conn::http1,
    service::service_fn,
    Request, Response,
};
use hyper_rustls::{ConfigBuilderExt, TlsAcceptor};
use hyper_util::{
    client::legacy::Client,
    rt::{TokioExecutor, TokioIo},
};
use std::{net::SocketAddr, sync::Arc};
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

    #[instrument]
    pub async fn serve_https(
        &mut self,
        addr: SocketAddr,
    ) -> Result<impl Future<Output = Result<()>>> {
        let tls = rustls::ClientConfig::builder()
            .with_safe_defaults()
            .with_native_roots()?
            .with_no_client_auth();
        let https = hyper_rustls::HttpsConnectorBuilder::new()
            .with_tls_config(tls)
            .https_only()
            .enable_http1()
            .build();

        /// Uhh not great but who cares tho
        let client: &'static _ =
            Box::leak(Box::new(Client::builder(TokioExecutor::new()).build(https)));
        let proxy = {
            move |req: Request<body::Incoming>| async move {
                info!("Got request: {req:?}. URI: {:?}", req.uri());

                let res = client.request(req).await?;

                info!("Got response: {res:?}");
                Ok::<_, anyhow::Error>(res)
            }
        };

        let cert_chain = vec![rustls::Certificate(self.cert_man.root.serialize_der()?)];
        let key_der = rustls::PrivateKey(self.cert_man.root.get_key_pair().serialize_der());

        let listener = TcpListener::bind(addr).await?;

        let mut acceptor = TlsAcceptor::builder()
            .with_single_cert(cert_chain, key_der)?
            .with_http11_alpn()
            .with_incoming(listener);

        let proxy = service_fn(proxy);

        let task: JoinHandle<Result<()>> = tokio::spawn(async move {
            loop {
                let (stream, _) = acceptor.accept().await?;
                tokio::spawn(async move {
                    if let Err(e) = http1::Builder::new().serve_connection(stream, proxy).await {
                        error!("Error handling connection: {}", e);
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

        Ok(task.map(|r: Result<Result<()>, JoinError>| match r {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e.into()),
            Err(e) => Err(e.into()),
        }))
    }
}
