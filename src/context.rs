use std::sync::Arc;

use crate::blob::Blob;
use crate::blob::BlobId;
use crate::generic::never;
use crate::queries::Request;
use crate::queries::Response;
use crate::storage::sqlite::SqliteStorage;

#[derive(Debug, Default)]
pub struct Context {
    storage: Option<Arc<SqliteStorage>>,
}

impl Context {
    pub fn new(storage: impl Into<Option<Arc<SqliteStorage>>>) -> Context {
        let storage = storage.into();
        Context { storage }
    }

    pub fn query(&mut self, request: Request) -> Result<Response, never> {
        todo!()
    }

    pub fn get_blob(&self, id: impl Into<BlobId>) -> Result<Option<Blob>, never> {
        todo!()
    }

    pub fn insert_blob(&self, data: impl Into<Blob>) -> Result<BlobId, never> {
        todo!()
    }

    pub fn get_responses(&self, request: Request) -> Result<never, never> {
        todo!()
    }

    pub fn insert_response(&self, request: Request, response: Response) {}
    fn insert_response_blob(&self, request: impl Into<BlobId>, response: impl Into<BlobId>) {}
}
