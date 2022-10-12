#![cfg(windows)]
#![cfg_attr(debug_assertions, allow(unused))]
use std::any::type_name;
use std::any::Any;
use std::any::TypeId;
use std::fmt::Debug;

use tracing::instrument;
use tracing::metadata::LevelFilter;
use tracing::trace as log;
use tracing_subscriber::fmt::format::FmtSpan;
use windows::core::InParam;
use windows::core::Interface;
use windows::core::HSTRING;
use windows::w;
use windows::Media::SpeechSynthesis::SpeechSynthesizer;
use windows::Media::SpeechSynthesis::VoiceInformation;
use windows::Storage::Streams::Buffer;
use windows::Storage::Streams::DataReader;
use windows::Storage::Streams::IBuffer;

use super::*;

pub async fn speak(text: &str) -> Result<Vec<u8>, eyre::Report> {
    speak_as(text, None).await
}

pub async fn speak_as(text: &str, voice_name: impl Into<Option<&str>>) -> Result<Vec<u8>, eyre::Report> {
        let synth = SpeechSynthesizer::new().wrap()?;

        let voices = Vec::from_iter(SpeechSynthesizer::AllVoices()?);

        // first, if we have a list of required voices, filter out all other voices
        // voice names can case-insensitive full matches for the voice name or
        // description, or suffix matches for the voice "ID".

        // then sort the candidates, with "preferred" voices first, with ties broken
        // by system default, then alphabetically by ID.

        dbg!(voices);

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
            .SynthesizeTextToStreamAsync(&HSTRING::from(text))
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

        Ok(bytes)
    }
