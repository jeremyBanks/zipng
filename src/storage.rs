mod layered;
mod no;
mod sqlite;

use std::fmt::Debug;

use async_trait::async_trait;
use miette::Diagnostic;
use static_assertions::assert_obj_safe;
use thiserror::Error;
use tracing::error;

pub use self::layered::*;
pub use self::no::*;
pub use self::sqlite::*;
use crate::blobs::Blip;
use crate::blobs::UnknownBlip;
use crate::blobs::UnknownBlob;
use crate::AnyRequest;
use crate::AnyResponse;
use crate::Blob;
use crate::Blobbable;
use crate::Metadata;
use crate::Response;
#[cfg(doc)]
use crate::*;

#[derive(Debug, Error, Diagnostic)]
#[error("{self:?}")]
pub enum StorageError {
    #[error("storage operation not supported")]
    Unsupported,
    #[error("storage operation not allowed")]
    Denied,
    #[error("storage operation failed")]
    Failed,
}

assert_obj_safe!(StorageImpl);

/// A storage backend implementation. This is a non-object-safe extension trait
/// over [`StorageImpl`].
impl<S> Storage for S where S: StorageImpl + ?Sized {}

#[async_trait]
pub trait Storage: StorageImpl {
    async fn insert_blob<T: Blobbable + ?Sized>(
        &self,
        blob: Blob<T>,
    ) -> Result<Blip<T>, StorageError> {
        // Ok(self.insert_blob_bytes(blob.retype())?.retype())
        todo!()
    }

    async fn get_blob<T: Blobbable + ?Sized>(
        &self,
        blip: Blip<T>,
    ) -> Result<Blob<T>, StorageError> {
        // Ok(self.get_blob_bytes(blip.retype())?.retype())
        todo!()
    }
}

pub struct ResponseItem<Request: crate::Request> {
    pub request: Blip<Request>,
    pub response: Blip<Request::Response>,
    pub metadata: Blip<Metadata>,
}

/// The storage backend for implementors. For the sake of object
/// safety, methods on this trait not generic, and typically operate on untyped
/// blips/blobs
#[async_trait]
pub trait StorageImpl: Debug + Send + Sync {
    async fn insert_blob_impl(&self, blob: &UnknownBlob) -> Result<UnknownBlip, StorageError> {
        Err(StorageError::Unsupported)
    }

    async fn get_blob_impl(&self, blip: UnknownBlip) -> Result<Option<UnknownBlob>, StorageError> {
        Err(StorageError::Unsupported)
    }

    async fn insert_response_impl(
        &self,
        request: UnknownBlip,
        response: UnknownBlip,
    ) -> Result<UnknownResponseItem, StorageError> {
        Err(StorageError::Unsupported)
    }

    async fn get_response_impl(
        &self,
        request: UnknownBlip,
    ) -> Result<Option<UnknownResponseItem>, StorageError> {
        Err(StorageError::Unsupported)
    }

    async fn get_responses_impl(
        &self,
        request: UnknownBlip,
    ) -> Result<Box<dyn Iterator<Item = UnknownResponseItem>>, StorageError> {
        Ok(Box::new(self.get_response_impl(request).await?.into_iter()))
    }
}

pub struct UnknownResponseItem {
    pub request: UnknownBlip,
    pub response: UnknownBlip,
    pub metadata: Blip<Metadata>,
}
