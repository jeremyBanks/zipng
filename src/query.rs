use std::fmt::Debug;

use async_trait::async_trait;
use derive_more::From;
use derive_more::TryInto;
use miette::Diagnostic;
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;

use crate::blob::Representable;
use crate::never;
use crate::panic;
use crate::Blob;
use crate::BlobId;

/// context associated with a given request instance.
///
/// all of a requests instance evaluation's interactions with the rest of the
/// engine go through its context.
pub struct Context<Request: self::Request> {
    request: Request,
}

impl<Request: self::Request + Sync> Context<Request> {
    pub fn populate(&mut self, synonym: Request) -> Result<(), never> {
        Ok(())
    }
}

#[async_trait]
pub trait Request: Debug + Serialize + Deserialize<'static> + Clone + Sync + Send {
    const TAG: u32;
    type Response: self::Response;
    type Error: Debug + Into<self::Error> + From<self::Error>;

    async fn execute(&self, context: &mut Context<Self>) -> Result<Self::Response, Self::Error> {
        Err(Error::NotSupported)?
    }
}

pub trait Response: Debug + Serialize + Deserialize<'static> + Clone + Sync + Send {
    type Request: self::Request;
}

#[derive(Debug, Serialize, Deserialize, Clone, TryInto, From)]
#[repr(u32)]
pub enum AnyRequest {
    Blob(BlobId<never>) = BlobId::<never>::TAG,
    TextToSpeech(TextToSpeech) = TextToSpeech::TAG,
}

#[derive(Debug, Serialize, Deserialize, Clone, TryInto, From)]
#[repr(u32)]
pub enum AnyResponse {
    Blob(Blob<never>) = BlobId::<never>::TAG,
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
impl<Representing: Representable + ?Sized> Request for BlobId<Representing> {
    const TAG: u32 = 0x00;
    type Response = Blob<Representing>;
    type Error = Error;
}

impl<Representing: Representable + ?Sized> Response for Blob<Representing> {
    type Request = BlobId<Representing>;
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TextToSpeech {
    text: BlobId<str>,
    language: Option<BlobId<str>>,
    voice_name: Option<BlobId<str>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TextToSpeechResponse {
    blob_id: BlobId<never>,
}

pub fn text_to_speech(text: impl ToString) -> TextToSpeech {}

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
                text: text.clone(),
                language: language.clone(),
                voice_name: None,
            })?;
        }
        if self.language.is_some() {
            context.populate(Self {
                text: text.clone(),
                language: None,
                voice_name: None,
            })?;
        }

        Ok(TextToSpeechResponse { blob_id: todo!() })
    }
}

impl Response for TextToSpeechResponse {
    type Request = TextToSpeech;
}
