use std::path::Path;
use std::sync::Arc;
use std::sync::RwLock;

use crate::never;
use crate::panic;
use crate::queries;

#[derive(Debug)]
pub struct Engine<Storage: crate::Storage> {
    storage: Arc<Storage>,
}

impl<Storage: crate::Storage> Engine<Storage> {
    pub fn new_in_memory() -> Arc<Engine<Storage>> {
        todo!()
    }

    pub fn new_in_file(path: impl Into<Path>) -> Arc<Engine<Storage>> {
        todo!()
    }

    fn execute<Request: crate::Request>(
        &self,
        request: Request,
    ) -> Result<Request::Response, never> {
        todo!()
    }

    pub fn get_blob(&self, id: impl Into<crate::BlobId>) -> Result<Option<crate::Blob>, never> {
        todo!()
    }

    pub fn http_get(&self, url: impl Into<String>) -> Result<panic, panic> {
        self.execute(queries::http_get::Request { url: url.into() })
    }
}

impl<Storage: crate::Storage> AsRef<Arc<Storage>> for Engine<Storage> {
    fn as_ref(&self) -> &Storage {
        &self.storage
    }
}
