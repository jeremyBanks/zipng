use serde::Deserialize;
use serde::Serialize;

use crate::blob::BlobId;
use crate::context::Context;
use crate::generic::never;

impl super::Request {
    pub fn text_to_speech(text: impl AsRef<str>) -> Result<Self, never> {
        Ok(Self::from(Request {
            text: BlobId::from_bytes(text.as_ref().as_bytes()),
            voice_name: None,
            language: Some(BlobId::from_bytes(b"en")),
        }))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Request {
    text: BlobId,
    voice_name: Option<BlobId>,
    language: Option<BlobId>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Response {}

pub fn query(request: &Request, context: &mut Context) -> Result<Response, never> {
    todo!()
}
