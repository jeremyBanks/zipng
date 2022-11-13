mod layered;
mod no;
mod sqlite;
mod web;

use std::fmt::Debug;

use miette::Diagnostic;
use static_assertions::assert_obj_safe;
use thiserror::Error;
use tracing::error;

pub use self::layered::*;
pub use self::no::*;
pub use self::sqlite::*;
pub use self::web::*;
use crate::blobs::bytes;
use crate::blobs::Blip;
use crate::Blob;
use crate::Blobbable;
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

assert_obj_safe!(Storage);

/// Generic wrapper methods for [`Storage`] need to go here because putting them
/// on the trait itself would break object-safety.
impl<S> StorageExt for S where S: Storage + ?Sized {}
pub trait StorageExt: Storage {
    fn insert_blob<T: Blobbable + ?Sized>(&self, blob: Blob<T>) -> Result<Blip<T>, StorageError> {
        Ok(self.insert_blob_bytes(blob.retype())?.retype())
    }

    fn get_blob<T: Blobbable + ?Sized>(&self, blip: Blip<T>) -> Result<Blob<T>, StorageError> {
        Ok(self.get_blob_bytes(blip.retype())?.retype())
    }
}

/// A storage backend for an `Engine`. This is object-safe and typically
/// handled as an `Arc<dyn Storage>`.
pub trait Storage: Debug + Send + Sync {
    fn insert_blob_bytes(&self, blob: Blob<bytes>) -> Result<Blip<bytes>, StorageError> {
        Err(StorageError::Unsupported)
    }

    fn get_blob_bytes(&self, blip: Blip<bytes>) -> Result<Blob<bytes>, StorageError> {
        Err(StorageError::Unsupported)
    }

    fn insert_query_bytes(&self, query: Query<bytes>) -> Result<(), StorageError> {
        Err(StorageError::Unsupported)
    }

    fn get_query_bytes(&self, request: ByteBlip) -> Result<Query<bytes>, StorageError> {
        Err(StorageError::Unsupported)
    }

    // fn insert_response_id(
    //     &self,
    //     request_id: Blip,
    //     response_id: Blip,
    // ) -> Result<(), StorageError> {
    //     Err(StorageError::Unsupported)
    // }

    // fn get_response_ids(
    //     &self,
    //     request_id: Blip,
    // ) -> Result<Box<dyn Iterator<Item = Result<ResponseIdRecord, StorageError>>>,
    // StorageError> {
    //     Err(StorageError::Unsupported)
    // }

    // fn get_response_id(
    //     &self,
    //     request_id: Blip,
    // ) -> Result<Option<ResponseIdRecord>, StorageError> {
    //     if let Some(result) = self.get_response_ids(request_id)?.next() {
    //         Ok(Some(result?))
    //     } else {
    //         Ok(None)
    //     }
    // }

    // fn insert_response<Request: crate::queries::Request>(
    //     &self,
    //     request: &Request,
    //     response: &Request::Response,
    // ) -> Result<(), StorageError> {
    //     todo!()
    // }

    // fn get_responses<Request: crate::queries::Request>(
    //     &self,
    //     request: &Request,
    // ) -> Result<
    //     Box<dyn Iterator<Item = Result<ResponseRecord<Request::Response>,
    // StorageError>>>,     StorageError,
    // > { todo!()
    // }

    // fn get_response<Request: crate::queries::Request>(
    //     &self,
    //     request: &Request,
    // ) -> Result<Option<ResponseRecord<Request::Response>>, StorageError> {
    //     if let Some(result) = self.get_responses(request)?.next() {
    //         Ok(Some(result?))
    //     } else {
    //         Ok(None)
    //     }
    // }

    // // XXX: move these to context!
    // // storage is internal and it can be simple and dumb.
    // fn get_responses_where<Request: crate::queries::Request>(
    //     &self,
    //     request: &Request,
    //     predicate: impl 'static + Fn(&ResponseRecord<Request::Response>) -> bool,
    // ) -> Result<
    //     Box<dyn Iterator<Item = Result<ResponseRecord<Request::Response>,
    // StorageError>>>,     StorageError,
    // > { let responses = self.get_responses(request)?;
    // > Ok(Box::new(responses.filter_map( move |response| match response {
    // > Ok(response) => if predicate(&response) { Some(Ok(response)) } else { None
    // > }, Err(error) => Some(Err(error)), }, )))
    // }

    // fn get_response_where<Request: crate::queries::Request>(
    //     &self,
    //     request: &Request,
    //     predicate: impl 'static + Fn(&ResponseRecord<Request::Response>) -> bool,
    // ) -> Result<Option<ResponseRecord<Request::Response>>, StorageError> {
    //     if let Some(result) = self.get_responses_where(request,
    // predicate)?.next() {         Ok(Some(result?))
    //     } else {
    //         Ok(None)
    //     }
    // }
}

#[derive(Debug, Clone)]
pub struct RequestRecord<Request: crate::Request> {
    pub response_blip: Blip<Request>,
    pub inserted_at: u32,
    pub validated_at: u32,
}
