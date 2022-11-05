use std::sync::Arc;

use crate::blob::Blob;
use crate::blob::BlobId;
use crate::generic::never;
use crate::storage::sqlite::SqliteStorage;
use crate::queries::AnyRequest;
use crate::queries::AnyResponse;
use thiserror::Error;

#[derive(Debug, Default)]
pub struct Context<Request: crate::Request = AnyRequest> {
    storage: Option<Arc<SqliteStorage>>,

    request_and_aliases: Vec<Request>,
}

#[derive(Debug, Error)]
#[error("{self:?}")]
pub enum ContextError {

}

impl<Request: crate::Request> Context<Request> {
    pub fn new(storage: impl Into<Option<Arc<SqliteStorage>>>) -> Self {
        let storage = storage.into();
        Context { storage, ..Default::default() }
    }

    pub fn query(&mut self, request: AnyRequest) -> Result<AnyResponse, never> {
        todo!()
    }

    pub fn get_blob(&self, id: impl Into<BlobId>) -> Result<Option<Blob>, never> {
        todo!()
    }

    pub fn insert_blob(&self, data: impl Into<Blob>) -> Result<BlobId, never> {
        todo!()
    }

    pub fn get_responses(&self, request: Request) -> Result<Request::Response, never> {
        todo!()
    }

    pub fn insert_response<OtherRequest: crate::Request>(&self, request: OtherRequest, response: OtherRequest::Response) {
        todo!()
    }

    /// Adds an alias request that will also be associated with this request's result.
    pub fn populate(&self, request: Request) {

    }
}
