#![allow(unsafe_code)]

pub mod sqlite;
pub mod web;

use std::fmt::Debug;

use thiserror::Error;
use tracing::error;

use crate::blob::Blob;
use crate::blob::BlobId;
use crate::queries::traits::Request;
use crate::queries::traits::Response;

#[derive(Debug, Error)]
#[error("{self:?}")]
pub enum StorageError {
    #[error("storage operation was not supported or not allowed")]
    Denied,
    #[error("storage operation failed")]
    Failed,
}

pub trait Storage: Debug + Clone {
    fn get_blob(&self, blob_id: BlobId) -> Result<Option<Blob>, StorageError> {
        Err(StorageError::Denied)
    }

    fn set_blob(&self, blob: Blob) -> Result<BlobId, StorageError> {
        Err(StorageError::Denied)
    }

    fn insert_response(
        &mut self,
        request_id: BlobId,
        response_id: BlobId,
    ) -> Result<BlobId, StorageError> {
        Err(StorageError::Denied)
    }

    fn get_response_ids(&mut self, request_id: BlobId) -> Result<Box<dyn 'static +  Iterator<Item=Result<ResponseIdRecord, StorageError>>>, StorageError> {
        Err(StorageError::Denied)
    }

    fn get_responses<Req: Request>(
        &mut self,
        request: Req,
    ) -> Result<Box<dyn 'static + Iterator<Item=Result<ResponseRecord<Req::Response>, StorageError>>>, StorageError> {
        let request_blob =
            Blob::from(postcard::to_stdvec(&request).map_err(|_| StorageError::Failed)?);

        let response_ids = self.get_response_ids(request_blob.id())?;

        Ok(Box::new(response_ids
            .map(move |response_id| {
                let response_id = response_id?;
                let response_blob = self.get_blob(response_id.blob_id)?.unwrap();
                let response: Req::Response =
                    postcard::from_bytes(&response_blob.as_ref()).map_err(|_| StorageError::Failed)?;
                Ok(ResponseRecord {
                    response,
                    inserted_at: response_id.inserted_at,
                    validated_at: response_id.validated_at,
                })
            })))
    }
}

#[derive(Debug, Clone)]
pub struct ResponseIdRecord {
    pub blob_id: BlobId,
    pub inserted_at: u32,
    pub validated_at: u32,
}

#[derive(Debug, Clone)]
pub struct ResponseRecord<Res: Response> {
    pub response: Res,
    pub inserted_at: u32,
    pub validated_at: u32,
}
