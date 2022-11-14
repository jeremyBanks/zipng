use std::sync::Arc;

use async_trait::async_trait;
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;

use crate::blobs::UnknownBlip;
use crate::execute::Incremental;
use crate::never;
use crate::storage::StorageImpl;
use crate::AnyRequest;
use crate::AnyResponse;
use crate::Blip;
use crate::Blob;
use crate::Engine;
use crate::PhantomType;
#[cfg(doc)]
use crate::Request;
#[cfg(doc)]
use crate::Response;
use crate::SqliteStorage;
use crate::Storage;

/// A context is associated with a [`Request`] instance and manages all of its
/// interactions with the rest of the [`Engine`]. If the request produces a new
/// [`Response`], the [`Context`] is consumed to produce its [`Metadata`].
#[derive(Debug)]
pub struct Context {
    storage: Arc<dyn Storage>,
    request: Blip<AnyRequest>,
    aliases: Vec<Blip<AnyRequest>>,
}

/// Metadata associated with the production of a given [`Response`].
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Metadata {
    validated_at: i64,
    created_at: i64,
    read: Vec<Blob<AnyRequest>>,
    written: Vec<Blob<AnyRequest>>,
}

#[derive(Debug, Error)]
#[error("{self:?}")]
pub enum ContextError {}

impl Context {
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

#[async_trait]
impl Incremental for Context {
    async fn get<Request: crate::Request>(
        &self,
        request: &Request,
    ) -> Result<Request::Response, Request::Error> {
        todo!()
    }
}
