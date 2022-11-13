use std::sync::Arc;

use static_assertions::assert_obj_safe;
use thiserror::Error;

use crate::blobs::bytes;
use crate::never;
use crate::AnyRequest;
use crate::AnyResponse;
use crate::Blip;
use crate::Blob;
use crate::Engine;
use crate::Request;
use crate::Response;
use crate::SqliteStorage;
use crate::Storage;

/// A context is associated with a [`Request`] instance and manages all of its
/// interactions with the rest of the [`Engine`]. If the request produces a new
/// [`Response`], the [`Context`] is consumed to produce its [`Metadata`].
#[derive(Debug, Default)]
pub struct Context<Request>
where Request: crate::Request
{
    storage: Option<Arc<dyn Storage>>,
    request: Request,
    aliases: Vec<Blip<bytes>>,
}

/// Metadata associated with the production of a given [`Response`].
pub struct Metadata<Request: crate::Request> {
    request: Request,
}

#[derive(Debug, Error)]
#[error("{self:?}")]
pub enum ContextError {}

impl<Request> Context<Request>
where Request: crate::Request
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
