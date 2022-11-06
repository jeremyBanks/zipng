use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::RwLock;

use tokio::runtime::Handle;

use crate::never;
use crate::panic;
use crate::Blob;

#[derive(Debug)]
pub struct Engine<Storage: crate::Storage> {
    storage: Arc<Storage>,
    runtime: tokio::runtime::Handle,
}

impl<Storage: crate::Storage> Default for Engine<Storage>
where
    Storage: Default,
{
    fn default() -> Self {
        Self {
            storage: Default::default(),
            runtime: Handle::current(),
        }
    }
}

impl<Storage: crate::Storage> Engine<Storage> {
    pub fn new(storage: Arc<Storage>) -> Engine<Storage> {
        Self {
            storage,
            runtime: Handle::current(),
        }
    }

    fn execute<Request: crate::Request>(
        &self,
        request: Request,
    ) -> Result<Request::Response, never> {
        todo!()
    }

    // pub fn get_blob(&self, id: impl Into<crate::BlobId>) ->
    // Result<Option<crate::Blob>, never> {     todo!()
    // }

    pub fn http_get(&self, url: impl Into<String>) -> Result<panic, panic> {
        todo!()
        // self.execute(queries::http_get::Request { url: url.into() })
    }

    pub async fn text_to_speech(&self, arg: &str) -> Result<Blob, panic> {
        todo!()
    }
}
