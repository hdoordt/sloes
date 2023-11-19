use async_trait::async_trait;
use futures::Stream;
use std::{io::ErrorKind, path::Path};
use tokio::sync::broadcast;

use anyhow::Result;
use tokio_stream::wrappers::{errors::BroadcastStreamRecvError, BroadcastStream};

pub mod config;
pub mod project;

pub struct Store<T, H> {
    data: T,
    handle: H,
    update_tx: broadcast::Sender<()>,
    update_rx: broadcast::Receiver<()>,
}

impl<T: Persistent + Default + Sized + Send + Sync> Store<T, T::Handle> {
    pub async fn load_or_default(handle: T::Handle) -> Result<Self> {
        let data = match T::load(&handle).await {
            Ok(conf) => conf,
            Err(e) => match e.downcast_ref::<tokio::io::Error>() {
                Some(e) if e.kind() == ErrorKind::NotFound => T::default(),
                _ => return Err(e),
            },
        };
        let (update_tx, update_rx) = broadcast::channel(64);
        Ok(Self {
            handle,
            data,
            update_tx,
            update_rx,
        })
    }

    pub async fn update<F: Fn(&mut T)>(&mut self, f: F) {
        f(&mut self.data);
        self.data.store(&self.handle).await;
        // We keep a copy of the corresponding receiver in Self,
        // thus this won't ever fail
        self.update_tx.send(()).unwrap();
    }
}

impl<T, H> Store<T, H> {
    pub fn subscribe(
        &self,
    ) -> impl Stream<Item = std::result::Result<(), BroadcastStreamRecvError>> {
        BroadcastStream::new(self.update_tx.subscribe())
    }
}

#[async_trait]
trait Persistent: Sized + Send + Sync {
    type Handle;

    async fn load(handle: &Self::Handle) -> Result<Self>;
    async fn store(&self, handle: &Self::Handle) -> Result<Self>;
}
