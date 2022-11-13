use super::Storage;
use super::StorageError;
use crate::blobs::bytes;
use crate::blobs::ByteBlip;
use crate::blobs::ByteBlob;
use crate::Blip;
use crate::Blob;
use crate::Metadata;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// No-op dummy storage backend.
pub struct NoStorage;

impl Storage for NoStorage {
    fn insert_blob_bytes(&self, blob: ByteBlob) -> Result<ByteBlip, StorageError> {
        Ok(blob.blip())
    }

    fn get_blob_bytes(&self, blip: ByteBlip) -> Result<ByteBlob, StorageError> {
        blip.inline_blob().ok_or(StorageError::Failed)
    }

    fn insert_query_bytes(&self, query: Query<ByteBlip>) -> Result<(), StorageError> {
        Ok(())
    }

    fn get_query_bytes(&self, request: ByteBlip) -> Result<Query<ByteBlip>, StorageError> {
        Ok(vec![])
    }
}

pub struct Query<Request: crate::Request = ByteBlip> {
    request: Request,
    response: Request::Response,
    metadata: Metadata,
}

pub struct StorageMetadata {
    /// The first timestamp at which this request-response pair was inserted in
    /// the database. Nanoseconds since epoch.
    inserted_at: u64,
    /// The most recent timestamp at which this request-response pair was
    /// inserted in the database (they're the same, but the query metadata may
    /// have changed). Nanoseconds since epoch.
    updated_at: u64,
}
