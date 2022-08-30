pub trait Tts: Default {
    fn text_to_speech(&self, text: &str) -> Result<Vec<u8>, miette::Report>;
}

pub fn tts() -> impl Tts {
    #[cfg(windows)]
    return windows::WindowsTts::default();

    #[cfg(not(windows))]
    return todo!();
}

#[cfg(windows)]
mod windows {
    #[derive(Default, Debug, Clone)]
    pub struct WindowsTts;

    impl crate::Tts for WindowsTts {
        fn text_to_speech(&self, text: &str) -> Result<Vec<u8>, miette::Report> {
            todo!()
        }
    }
}
