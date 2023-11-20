mod cert;
mod error;
mod proxy;
mod storage;

use std::sync::Arc;

use sluus_ui;
use tracing::{info, level_filters::LevelFilter, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer};

use crate::{
    cert::CertManager,
    proxy::Proxy,
    storage::{config::ConfigStore, project::ProjectStore},
};

#[tokio::main]
pub async fn main() {
    println!("Hello!");

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_filter(LevelFilter::from_level(Level::DEBUG)))
        .init();

    info!("Hello proxy!");

    let config = ConfigStore::default();
    let config = Arc::new(config);

    let project = ProjectStore::default();
    let project = Arc::new(project);
    let mut proxy = Proxy::new(
        config.clone(),
        project,
        CertManager::load_or_generate(config.clone()).await.unwrap(),
    );
    proxy
        .serve_http("127.0.0.1:9001".parse().unwrap())
        .await
        .unwrap()
        .await
        .unwrap();

    // sluus_ui::run_it();
    // TODO
    // Enable logging
    // load config
    // load or generate root cert
    // start proxy
    // start ui
    //    contains instructions on how to download and configure root cert
    // profit!
}
