use async_trait::async_trait;
use std::path::{Path, PathBuf};

use anyhow::Result;

use super::{Persistent, Store};

pub type ConfigStore = Store<Config, PathBuf>;

pub struct Config {
    project_path: PathBuf,
    root_cert_path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        todo!("Storage path default is in config folder, Root cert path is somewhere tbd?");
    }
}

#[async_trait]
impl Persistent for Config {
    type Handle = PathBuf;
    async fn load(path: &PathBuf) -> Result<Self> {
        Ok(Config::default())
    }

    async fn store(&self, path: &PathBuf) -> Result<Self> {
        todo!()
    }
}
