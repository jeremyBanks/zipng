use serde::Deserialize;
use serde::Serialize;

use crate::blob::BlobId;
use crate::context::Context;
use crate::generic::never;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Request {
    blob_ids: Vec<BlobId>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Response {
    blob_id: BlobId,
}

pub fn query(request: &Request, context: &mut Context) -> Result<Response, never> {
    let mut bytes: Vec<u8> = Vec::new();
    for blob_id in request.blob_ids.iter() {
        let blob = context.get_blob(blob_id).unwrap().unwrap();
        bytes.extend(blob.as_ref());
    }
    let blob_id = context.insert_blob(bytes)?;
    Ok(Response { blob_id })
}
