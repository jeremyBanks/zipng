use std::fmt::Debug;

use async_trait::async_trait;
use derive_more::From;
use derive_more::TryInto;
use miette::Diagnostic;
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;

use super::RequestError;
use crate::blobs::bytes;
use crate::blobs::Postcard;
use crate::blobs::PostcardBlob;
use crate::never;
use crate::panic;
use crate::storage::Storage;
use crate::Blip;
use crate::Blob;
use crate::Blobbable;
use crate::Context;
use crate::Engine;
use crate::Request;
use crate::Response;
#[cfg(doc)]
use crate::*;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TextToSpeech {
    pub text: Blip<str>,
    pub language: Option<Blip<str>>,
    pub voice_name: Option<Blip<str>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TextToSpeechResponse {
    pub speech: Blob<bytes>,
}

#[async_trait]
impl Request for TextToSpeech {
    const TAG: u32 = 's' as _;
    type Response = TextToSpeechResponse;
    async fn execute(&self, context: &mut Context) -> Result<Self::Response, RequestError> {
        let Self {
            text,
            language,
            voice_name,
        } = self;

        if self.voice_name.is_some() {
            context.populate(Self {
                text: *text,
                language: *language,
                voice_name: None,
            });
        }
        if self.language.is_some() {
            context.populate(Self {
                text: *text,
                language: None,
                voice_name: None,
            });
        }

        Ok(TextToSpeechResponse { speech: todo!() })
    }
}

impl Response for TextToSpeechResponse {
    type Request = TextToSpeech;
}
