use super::StorageError;
use super::StorageImpl;
use super::UnknownResponseItem;
use crate::blobs::bytes;
use crate::blobs::UnknownBlip;
use crate::blobs::UnknownBlob;
use crate::Blip;
use crate::Blob;
use crate::Metadata;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// No-op dummy storage backend. Retains nothing, without error.
pub struct NoStorage;

impl StorageImpl for NoStorage {
    fn insert_blob_impl(&self, blob: &UnknownBlob) -> Result<UnknownBlip, StorageError> {
        Ok(blob.blip())
    }

    fn get_blob_impl(&self, blip: UnknownBlip) -> Result<Option<UnknownBlob>, StorageError> {
        Ok(None)
    }

    fn insert_response_impl(
        &self,
        request: UnknownBlip,
        response: UnknownBlip,
    ) -> Result<UnknownResponseItem, StorageError> {
        Ok(UnknownResponseItem {
            request,
            response,
            metadata: Blip::new(Metadata::default()),
        })
    }

    fn get_response_impl(
        &self,
        request: UnknownBlip,
    ) -> Result<Option<UnknownResponseItem>, StorageError> {
        Ok(None)
    }
}
