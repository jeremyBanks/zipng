use std::rc::Rc;
use std::fmt::Debug;
use async_trait::async_trait;
use std::any::Any;

mod windows;

#[async_trait]
pub trait Tts {
    async fn text_to_speech(&self, text: &str) -> Result<Speech, eyre::Report>;
}

pub fn tts() -> impl Tts {
    #![allow(unused)]

    #[cfg(windows)]
    return windows::WindowsTts::default();

    return unimplemented!("TTS is not available");
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Speech {
    pub text: String,
    pub audio: Vec<u8>,
}
