use std::path::{Path, PathBuf};

use anyhow::Result;

pub struct Config {
    pub storage_path: PathBuf,
    pub root_cert_path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        todo!("Storage path default is in config folder, Root cert path is somewhere tbd?");
    }
}

impl Config {
    pub async fn load(path: impl AsRef<Path>) -> Result<Self> {
        todo!()
    }
}
