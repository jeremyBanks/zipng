use serde::Deserialize;
use serde::Serialize;

use crate::blob::Blip;
use crate::context::Context;
use crate::generic::never;

impl super::Request {
    pub fn text_to_speech(text: impl AsRef<str>) -> Result<Self, never> {
        Ok(Self::from(Request {
            text: Blip::from_bytes(text.as_ref().as_bytes()),
            voice_name: None,
            language: Some(Blip::from_bytes(b"en")),
        }))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Request {
    text: Blip,
    voice_name: Option<Blip>,
    language: Option<Blip>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Response {}

pub fn query(request: &Request, context: &mut Context) -> Result<Response, never> {
    if request.voice_name.is_some() {
        context.alias(Request {
            text: request.text,
            voice_name: None,
            language: request.language,
        });
    }
    if request.language.is_some() {
        context.alias(Request {
            text: request.text,
            voice_name: None,
            language: None,
        });
    }

    todo!()
}
