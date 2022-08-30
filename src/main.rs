#![cfg_attr(debug_assertions, allow(unused))]

use std::any::type_name;
use std::any::Any;
use std::any::TypeId;
use std::fmt::Debug;

use async_executor::LocalExecutor;
use futures_lite::future;
use tracing::instrument;
use tracing::metadata::LevelFilter;
use tracing::trace as log;
use tracing_subscriber::fmt::format::FmtSpan;
use windows::core::InParam;
use windows::core::Interface;
use windows::w;
use windows::Media::SpeechSynthesis::SpeechSynthesizer;
use windows::Storage::Streams::Buffer;
use windows::Storage::Streams::DataReader;
use windows::Storage::Streams::IBuffer;

use self::fic::*;
use self::sept::*;
use self::tts::*;

mod fic;
mod sept;
mod tts;

#[instrument]
fn main() -> Result<(), miette::Report> {
    tracing::subscriber::set_global_default(
        tracing_subscriber::fmt()
            .with_max_level(LevelFilter::TRACE)
            .with_target(false)
            .with_level(false)
            .with_span_events(FmtSpan::FULL)
            .with_file(true)
            .with_line_number(true)
            .without_time()
            .finish(),
    )
    .wrap()?;

    let executor = LocalExecutor::new();

    future::block_on(executor.run(async { app().await }))
}

#[instrument]
async fn app() -> Result<(), miette::Report> {
    test();

    let synth = SpeechSynthesizer::new().wrap()?;

    let voice = synth.Voice().wrap()?;
    let options = synth.Options().wrap()?;

    log!(
        "      Language: {:>19}",
        voice.Language().wrap()?.to_string()
    );
    log!(
        "         Voice: {:>19}",
        voice.DisplayName().wrap()?.to_string()
    );
    log!("         Pitch: {:>19.2}", options.AudioPitch().wrap()?);
    log!("        Volume: {:>19.2}", options.AudioVolume().wrap()?);
    log!("         Speed: {:>19.2}", options.SpeakingRate().wrap()?);
    log!(
        "          Rest: {:>16?}",
        options.PunctuationSilence().wrap()?.0
    );
    log!(
        "           End: {:>16?}",
        options.AppendedSilence().wrap()?.0
    );
    log!(
        "         Words: {:>19}",
        options.IncludeWordBoundaryMetadata().wrap()?.to_string()
    );
    log!(
        "         Stops: {:>19}",
        options
            .IncludeSentenceBoundaryMetadata()
            .wrap()?
            .to_string()
    );

    let stream = synth
        .SynthesizeTextToStreamAsync(w!("hello, world!"))
        .wrap()?
        .await
        .wrap()?;

    let buffer = Buffer::Create(64 * 1024 * 1024)
        .wrap()?
        .cast::<IBuffer>()
        .wrap()?;

    stream
        .ReadAsync(
            InParam::from(Some(&buffer)),
            buffer.Capacity().unwrap(),
            Default::default(),
        )
        .wrap()?
        .await
        .wrap()?;

    let content_type = stream.ContentType().wrap()?;

    log!("        Length: {:>18}B", buffer.Length().wrap()?);
    log!("          Type: {:>19}", content_type.to_string());

    let mut bytes = vec![0u8; buffer.Length().wrap()? as usize];
    DataReader::FromBuffer(InParam::from(Some(&buffer)))
        .wrap()?
        .ReadBytes(&mut bytes)
        .wrap()?;

    std::fs::create_dir_all("./target/audio").wrap()?;
    std::fs::write("./target/audio/hello.wav", bytes).wrap()?;

    Ok(())
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
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
            },
        }
    }
}
