use std::sync::Arc;

use crate::blob::Blob;
use crate::blob::BlobId;
use crate::storage::Storage;

#[derive(Debug, Default)]
pub struct Context {
    storage: Option<Arc<Storage>>,
}

impl Context {
    pub fn new(storage: impl Into<Option<Arc<Storage>>>) -> Context {
        let storage = storage.into();
        Context { storage }
    }

    pub fn get_blob(&self, id: BlobId) -> Option<Blob> {
        None
    }

    pub fn insert_blob(&mut self, data: impl Into<Blob>) {}
}
