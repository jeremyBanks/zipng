use std::rc::Rc;

mod windows;

pub trait Tts {
    fn obj(self) -> Rc<dyn Tts>
    where
        Self: Sized + 'static,
    {
        Rc::new(self)
    }

    fn text_to_speech(&self, text: &str) -> Result<Speech, eyre::Report>;
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
