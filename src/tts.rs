use std::rc::Rc;

pub trait Tts {
    fn obj(self) -> Rc<dyn Tts>
    where
        Self: Sized + 'static,
    {
        Rc::new(self)
    }

    fn text_to_speech(&self, text: Rc<str>) -> Result<Speech, eyre::Report>;
}

pub fn tts() -> impl Tts {
    #![allow(unused)]

    #[cfg(windows)]
    return windows::WindowsTts::default();

    return unimplemented!();
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Speech {
    pub text: Rc<str>,
    pub audio: Rc<[u8]>,
}

#[cfg(windows)]
mod windows {
    use std::rc::Rc;

    use super::Speech;

    #[derive(Default, Debug, Clone)]
    pub struct WindowsTts;

    impl crate::Tts for WindowsTts {
        fn text_to_speech(&self, text: Rc<str>) -> Result<Speech, eyre::Report> {
            Ok(Speech {
                text,
                audio: todo!(),
            })
        }
    }
}
