#![allow(unsafe_code)]

mod baked;
pub mod layered;
pub mod sqlite;
pub mod web;

use std::fmt::Debug;

use thiserror::Error;
use tracing::error;

use crate::blob::Blob;
use crate::blob::BlobId;
use crate::blob::Representable;

#[derive(Debug, Error)]
#[error("{self:?}")]
pub enum StorageError {
    #[error("storage operation not supported")]
    Unsupported,
    #[error("storage operation not allowed")]
    Denied,
    #[error("storage operation failed")]
    Failed,
}

pub trait Storage: Debug + Clone + Send {
    fn get_blob<Rep: Representable>(&self, blob_id: BlobId<Rep>) -> Result<Option<Blob<Rep>>, StorageError> {
    }

    // fn insert_blob(&self, blob: Blob) -> Result<BlobId, StorageError> {
    //     Err(StorageError::Unsupported)
    // }

    // fn insert_response_id(
    //     &self,
    //     request_id: BlobId,
    //     response_id: BlobId,
    // ) -> Result<(), StorageError> {
    //     Err(StorageError::Unsupported)
    // }

    // fn get_response_ids(
    //     &self,
    //     request_id: BlobId,
    // ) -> Result<Box<dyn Iterator<Item = Result<ResponseIdRecord, StorageError>>>, StorageError>
    // {
    //     Err(StorageError::Unsupported)
    // }

    // fn get_response_id(
    //     &self,
    //     request_id: BlobId,
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
    //     Box<dyn Iterator<Item = Result<ResponseRecord<Request::Response>, StorageError>>>,
    //     StorageError,
    // > {
    //     todo!()
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
    //     Box<dyn Iterator<Item = Result<ResponseRecord<Request::Response>, StorageError>>>,
    //     StorageError,
    // > {
    //     let responses = self.get_responses(request)?;
    //     Ok(Box::new(responses.filter_map(
    //         move |response| match response {
    //             Ok(response) =>
    //                 if predicate(&response) {
    //                     Some(Ok(response))
    //                 } else {
    //                     None
    //                 },
    //             Err(error) => Some(Err(error)),
    //         },
    //     )))
    // }

    // fn get_response_where<Request: crate::queries::Request>(
    //     &self,
    //     request: &Request,
    //     predicate: impl 'static + Fn(&ResponseRecord<Request::Response>) -> bool,
    // ) -> Result<Option<ResponseRecord<Request::Response>>, StorageError> {
    //     if let Some(result) = self.get_responses_where(request, predicate)?.next() {
    //         Ok(Some(result?))
    //     } else {
    //         Ok(None)
    //     }
    // }
}

#[derive(Debug, Clone)]
pub struct RequestRecord<Request: crate::Request> {
    pub response_blob_id: BlobId<Request>,
    pub inserted_at:      u32,
    pub validated_at:     u32,
}
