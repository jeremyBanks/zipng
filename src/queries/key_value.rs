use serde::Deserialize;
use serde::Serialize;

use crate::context::Context;
use crate::storage::StorageError;
use crate::blob::BlobId;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Request {
    key_path: Vec<BlobId>
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Response {
    value: BlobId
}

pub fn query(request: &Request, context: &mut Context) -> Result<Response, StorageError> {
    Err(StorageError::Unsupported)
}
