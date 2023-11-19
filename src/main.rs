mod cert;
mod config;
mod error;
mod proxy;
mod storage;

use sluus_ui;

#[tokio::main]
pub async fn main() {
    println!("Hello!");

    sluus_ui::run_it();
    // TODO
    // Enable logging
    // load config
    // load or generate root cert
    // start proxy
    // start ui
    //    contains instructions on how to download and configure root cert
    // profit!
}
