use serde::Deserialize;
use serde::Serialize;

use super::traits::Request;
use super::traits::Response;
use super::Context;
use crate::blob::Blob;
use crate::blob::BlobId;

impl super::Request {
    pub fn blob(blob_id: BlobId) -> Self {
        Self::Blob(BlobRequest { blob_id })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct BlobRequest {
    blob_id: BlobId,
}

impl Request for BlobRequest {
    type Response = BlobResponse;

    fn query(&self, context: &mut Context) -> Self::Response {
        BlobResponse {
            data: context.get_blob(self.blob_id),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct BlobResponse {
    data: Option<Blob>,
}

impl Response for BlobResponse {
    fn max_age_seconds(&self) -> u32 {
        Self::NO_SAVE
    }
}
