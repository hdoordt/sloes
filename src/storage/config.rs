use async_trait::async_trait;
use std::path::{Path, PathBuf};

use anyhow::Result;

use super::{Persistent, Store};

pub type ConfigStore = Store<Config, PathBuf>;

#[derive(Debug)]
#[non_exhaustive]
pub struct Config {
    pub project_path: PathBuf,
    pub root_cert_path: PathBuf,
    pub root_key_path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            project_path: "".into(),
            root_cert_path: "../ca.pem".into(),
            root_key_path: "ca.key".into(),
        }
        // Storage path default is in config folder, Root cert path is somewhere tbd?
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
