#![allow(unsafe_code)]

pub mod sqlite;
pub mod web;
pub mod layered;

use std::fmt::Debug;

use thiserror::Error;
use tracing::error;

use crate::blob::Blob;
use crate::blob::BlobId;
use crate::queries::traits::Response;

#[derive(Debug, Error)]
#[error("{self:?}")]
pub enum StorageError {
    #[error("storage operation was not supported or not allowed")]
    Denied,
    #[error("storage operation failed")]
    Failed,
}

pub trait Storage: Debug + Clone + Send {
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

    fn get_response_ids(
        &mut self,
        request_id: BlobId,
    ) -> Result<Box<dyn Iterator<Item = Result<ResponseIdRecord, StorageError>>>, StorageError>
    {
        Err(StorageError::Denied)
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
