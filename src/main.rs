use std::{
    any::{type_name, Any, TypeId},
    fmt::Debug,
};
use tracing::info;
use windows::{
    core::InParam, w, Media::SpeechSynthesis::SpeechSynthesizer, Storage::Streams::Buffer,
};

#[tracing::instrument]
fn main() -> Result<(), miette::Report> {
    tracing::subscriber::set_global_default(tracing_subscriber::FmtSubscriber::new()).wrap()?;

    let voices = SpeechSynthesizer::AllVoices().wrap()?;

    for voice in voices {
        info!("{}", voice.DisplayName().wrap()?);
    }

    let synth = SpeechSynthesizer::new().wrap()?;

    let stream = synth
        .SynthesizeTextToStreamAsync(w!("hello, world!"))
        .wrap()?
        .get()
        .wrap()?;

    let content_type = stream.ContentType().wrap()?;

    let buffer = stream
        .ReadAsync(InParam::null(), 0, Default::default())
        .wrap()?
        .get()
        .wrap()?;

    dbg!(buffer);

    Ok(())
}

/// A type used to wrap arbitrary Debuggable error types as result Diagnostics.
///
/// This shouldn't be used on types that already implement Diagnostic.
#[derive(Debug, thiserror::Error, miette::Diagnostic, derive_more::From)]
#[error("{0}{1}")]
pub(crate) struct WrappedError(&'static str, String);

impl WrappedError {
    pub fn new(msg: &'static str, err: impl Debug) -> Self {
        Self(msg, format!("{err:?}"))
    }
}

pub(crate) trait DebugResultExt {
    type Ok;
    fn wrap(self) -> Result<Self::Ok, WrappedError>;

    fn unwrap(self) -> Self::Ok
    where
        Self: Sized,
    {
        self.wrap().unwrap()
    }
}

impl<T, E: Debug + Any> DebugResultExt for Result<T, E> {
    type Ok = T;
    fn wrap(self) -> Result<T, WrappedError> {
        match self {
            Ok(ok) => Ok(ok),
            Err(err) => {
                let prefix = if err.type_id() == TypeId::of::<windows::core::Error>() {
                    "windows::core::"
                } else {
                    let prefix = type_name::<E>();
                    &prefix[..prefix.rfind("::").map(|x| x + 2).unwrap_or(prefix.len())]
                };
                Err(WrappedError::new(prefix, err))
            }
        }
    }
}
