use std::fmt::Debug;
use std::fmt::Display;

use async_trait::async_trait;
use miette::Diagnostic;
use thiserror::Error;

use crate::blobs::blip;
use crate::blobs::blip::blip;
use crate::blobs::blob;
use crate::blobs::Blip;
use crate::blobs::Blob;
use crate::blobs::BlobSerialization;
use crate::never;
use crate::panic;
use crate::query;
use crate::storage::StorageError;
use crate::Blobbable;
use crate::Request;

#[derive(Debug, Error, Diagnostic)]
#[error("{self:?}")]
pub enum IncrementalError {
    NotSupported,
    Denied,
    StorageError(StorageError),
    Failed(#[from] eyre::Report),
}

/// Common interface for `Engine`, `Storage`, and `Context`.
#[async_trait]
pub trait Incremental {
    type Error: Display + Debug + Into<IncrementalError> + From<IncrementalError>;

    /// Stores this [`Blob`] if it's too long to inline, and returns the
    /// respective [`Blip`].
    async fn insert_blob<T: Blobbable + ?Sized>(blob: Blob<T>) -> Result<Blip<T>, Self::Error> {
        Err(IncrementalError::NotSupported.into())
    }

    /// Retrieves the [`Blob`] corresponding to the given [`Blip`], if it's
    /// stored.
    async fn get_blob<T: Blobbable + ?Sized>(
        blip: Blip<T>,
    ) -> Result<Option<Blob<T>>, Self::Error> {
        Err(IncrementalError::NotSupported.into())
    }

    /// Get a [`Response`] for the given [`Request`], either from storage or by
    /// executing it.
    async fn get<Request: crate::Request>(
        &self,
        request: &Request,
    ) -> Result<Request::Response, Self::Error> {
        Err(IncrementalError::NotSupported.into())
    }

    /// Get a [`Response`] for the given [`Request`], either from storage or by
    /// executing it.
    async fn set<Request: crate::Request>(
        &self,
        request: &Request,
        response: &Request::Response,
    ) -> Result<(), Self::Error> {
        Err(IncrementalError::NotSupported.into())
    }

    /// Get a [`Blip`] of [`Response`] for the given [`Request`] [`Blip`],
    /// either from storage or by executing it if possible.
    async fn get_blip<Request: crate::Request>(
        &self,
        request: Blip<Request>,
    ) -> Result<Blip<Request::Response>, Self::Error> {
        Err(IncrementalError::NotSupported.into())
    }

    /// Sets the [`Response`] [`Blip`] for the given [`Request`] [`Blip`].
    async fn set_blip<Request: crate::Request>(
        &self,
        request: Blip<Request>,
        response: Blip<Request>,
    ) -> Result<(), Self::Error> {
        Err(IncrementalError::NotSupported.into())
    }
}
