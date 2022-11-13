use std::fmt::Debug;

use async_trait::async_trait;
use derive_more::From;
use derive_more::TryInto;
use miette::Diagnostic;
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;

use crate::blobs::bytes;
use crate::blobs::Postcard;
use crate::never;
use crate::panic;
use crate::Blip;
use crate::Blob;
use crate::Blobbable;
use crate::Context;
#[cfg(doc)]
use crate::Engine;

#[async_trait]
/// The input for an operation that can be executed by the [`Engine`].
pub trait Request: Default + Debug + Blobbable<Postcard> + Clone + Sync + Send {
    const TAG: u32;
    type Response: self::Response;
    type Error: Debug + Into<self::Error> + From<self::Error>;

    async fn execute(&self, context: &mut Context<Self>) -> Result<Self::Response, Self::Error> {
        Err(Error::NotSupported)?
    }
}

///  The output of an operation executed by the [`Engine`].
pub trait Response: Default + Debug + Blobbable<Postcard> + Clone + Sync + Send {
    type Request: self::Request;
}

/// An enum that may contain any [`Request`] type.
#[derive(Debug, Serialize, Deserialize, Clone, TryInto, From)]
#[repr(u32)]
pub enum AnyRequest {
    Blob(Blip<bytes>) = Blip::<bytes>::TAG,
    TextToSpeech(TextToSpeech) = TextToSpeech::TAG,
}

/// An enum that may contain any [`Response`] type.
#[derive(Debug, Serialize, Deserialize, Clone, TryInto, From)]
#[repr(u32)]
pub enum AnyResponse {
    Blob(Blob<bytes>) = Blip::<bytes>::TAG,
    TextToSpeech(TextToSpeechResponse) = TextToSpeech::TAG,
}

#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    #[error("parse or type error while deserializing")]
    NotRecognized,

    #[error("the engine does not support this request. results are only available if pre-loaded.")]
    NotSupported,

    #[error("403 Forbidden")]
    NotAllowed,

    #[error(transparent)]
    OtherFailure(#[from] eyre::Report),
}

impl From<panic> for Error {
    fn from(_: panic) -> Self {
        unreachable!()
    }
}

impl From<never> for Error {
    fn from(_: never) -> Self {
        unreachable!()
    }
}

#[async_trait]
impl<T> Request for Blip<T>
where T: ?Sized
{
    const TAG: u32 = 0x00;
    type Response = Blob<T>;
    type Error = Error;
}

impl<T> Response for Blob<T>
where T: ?Sized
{
    type Request = Blip<T>;
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TextToSpeech {
    text: Blip<str>,
    language: Option<Blip<str>>,
    voice_name: Option<Blip<str>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TextToSpeechResponse {
    blip: Blip<never>,
}

#[async_trait]
impl Request for TextToSpeech {
    const TAG: u32 = 0x31;
    type Response = TextToSpeechResponse;
    type Error = panic;
    async fn execute(&self, context: &mut Context<Self>) -> Result<Self::Response, panic> {
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

        Ok(TextToSpeechResponse { blip: todo!() })
    }
}

impl Response for TextToSpeechResponse {
    type Request = TextToSpeech;
}