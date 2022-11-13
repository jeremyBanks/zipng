use std::sync::Arc;

use thiserror::Error;

use crate::blobs::bytes;
use crate::never;
use crate::AnyRequest;
use crate::AnyResponse;
use crate::Blip;
use crate::Blob;
use crate::SqliteStorage;

#[derive(Debug, Default)]
pub struct Context<Request, Storage>
where
    Request: crate::Request,
    Storage: crate::Storage,
{
    storage: Storage,
    request: Request,
    aliases: Vec<Blip<bytes>>,
}

pub struct Metadata<Request: crate::Request> {
    request: Request,
}

#[derive(Debug, Error)]
#[error("{self:?}")]
pub enum ContextError {}

impl<Request, Storage> Context<Request, Storage>
where
    Request: crate::Request,
    Storage: crate::Storage,
{
    pub fn new(storage: impl Into<Option<Arc<SqliteStorage>>>) -> Self {
        let storage = storage.into();
        todo!()
    }

    pub fn query(&mut self, request: AnyRequest) -> Result<AnyResponse, never> {
        todo!()
    }

    pub fn get_blob<Rep>(&self, id: impl Into<Blip<Rep>>) -> Result<Option<Blob<Rep>>, never> {
        todo!()
    }

    pub fn insert_blob<Rep>(&self, data: impl Into<Blob<Rep>>) -> Result<Blip<Rep>, never> {
        todo!()
    }

    pub fn get_responses(&self, request: Request) -> Result<Request::Response, never> {
        todo!()
    }

    pub fn insert_response<OtherRequest: crate::Request>(
        &self,
        request: OtherRequest,
        response: OtherRequest::Response,
    ) {
        todo!()
    }

    /// Adds an alias request that will also be associated with this request's
    /// result.
    pub fn populate(&self, request: Request) {}
}
