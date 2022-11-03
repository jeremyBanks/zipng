use serde::Deserialize;
use serde::Serialize;

use crate::context::Context;
use crate::generic::never;
use crate::blob::BlobId;
use crate::blob::Blob;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Request {
    blob_id: BlobId
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(transparent)]
pub struct Response(Blob);

pub fn query(request: &Request, context: &mut Context) -> Result<Response, never> {
    todo!()
}
