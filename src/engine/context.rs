use std::sync::Arc;

use async_trait::async_trait;
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;

use crate::blobs::UnknownBlip;
use crate::execute::Incremental;
use crate::execute::IncrementalError;
use crate::never;
use crate::storage::StorageError;
use crate::storage::StorageImpl;
use crate::AnyRequest;
use crate::AnyResponse;
use crate::Blip;
use crate::Blob;
use crate::Engine;
use crate::PhantomType;
use crate::Request;
use crate::Response;
use crate::SqliteStorage;
use crate::Storage;

/// A context is associated with a [`Request`] instance and manages all of its
/// interactions with the rest of the [`Engine`]. If the request produces a new
/// [`Response`], the [`Context`] is consumed to produce its [`Metadata`].
#[derive(Debug)]
pub struct Context {
    /// A Blip representing the request this context is associated with.
    pub(in crate::engine) request: Blip<AnyRequest>,
    pub(in crate::engine) engine: Arc<Engine>,
    pub(in crate::engine) aliases: Vec<Blip<AnyRequest>>,
}

/// Metadata associated with the production of a given [`Response`].
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Metadata {
    pub validated_at: i64,
    pub created_at: i64,
    pub read: Vec<Blob<AnyRequest>>,
    pub written: Vec<Blob<AnyRequest>>,
}

impl Context {
    /// Adds an alias request that will also be associated with this request's
    /// result.
    pub fn alias(&self, request: AnyRequest) {}
}

#[async_trait]
impl Incremental for Context {
    type Error = IncrementalError;
}
