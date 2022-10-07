use std::convert::Infallible;
use std::marker::PhantomData;
use std::io::Write;
use std::io::Read;
use std::sync::Arc;



fn tempdir() -> Result<impl AsRef<std::path::Path>, std::io::Error> {
    tempfile::tempdir_in("target/ffmpegging")
}

#[doc(hidden)]
#[macro_export]
macro_rules! run {
    ($name:ident $(-$flag:ident $($expr:expr)? ),+) => {
        $crate::run!($name $(concat!("-", stringify!($flag)) $(,$expr)?),+)
    };
    ($name:ident $($flag:ident = $expr:expr $(,)?$(;)? )+) => {
        $crate::run!($name $(concat!("-", stringify!($flag)) ,$expr),+)
    };
    ($name:ident $($flag:ident: $expr:expr $(,)?$(;)? )+) => {
        $crate::run!($name $(concat!("-", stringify!($flag)) ,$expr),+)
    };
    ($name:ident $($expr:expr),*) => {
        ::duct::cmd!({
            static PATH: ::once_cell::sync::Lazy<::std::path::PathBuf> = ::once_cell::sync::Lazy::new(|| {
                let name = stringify!($name);
                if let Ok(name) = ::which::which(&name) {
                    if let Ok(name) = ::std::fs::canonicalize(&name) {
                        name
                    } else {
                        name
                    }
                } else {
                    name.into()
                }
            });
            &*PATH
        }, $($expr),*)
    };
}

pub fn ffmpeg(inputs: Vec<Input<Vec<u8>>, outputs: Vec<Input<Vec<u8>>) -> Result<duct::Expression, eyre::Report> {
    let dir = tempdir()?.as_ref();



    todo!()
}

fn test() {
    let title = vec![0u8, 1000];
    let body = vec![0u8, 1000];

    macro_rules! input { ($($tt:tt)*) => {{}} }
    macro_rules! output { ($($tt:tt)*) => {{}} }

    let title = input(title).dot("wav").build();
    let body = input(body).dot("wav").build();

    let mut output = output().dot("mka").audio(
        audio()
            .codec("opus")
            .channels(1)
            .bitrate(32 * 1024)
    );

    let outputs = HashMap::<Arc<Input>, Vec<u8>>::new();

    todo()!
}

#[derive(Debug, Clone, Default)]
pub struct Input {
    pub bytes: Vec<u8>,
    pub dot: Option<String>,
    pub container: Option<String>,
    pub video: Option<Vec<VideoStream>>,
    pub audio: Option<Vec<AudioStream>>,
    pub text: Option<Vec<TextStream>>,
}

pub fn input(bytes: Vec<u8>) -> Input {
    Input::new(bytes)
}

impl Input {
    pub fn new(bytes: Vec<u8>) -> Self {
        todo!()
    }

    pub fn dot(self, dot: impl ToString) -> Self {
        self.dot = Some(dot.to_string());
        self
    }

    pub fn build(self) -> Arc<Self> {
        Arc::new(self)
    }

    pub fn id(self: &Arc<Self>) -> u64 {
        Arc::as_ptr(self) as u64
    }
}

#[derive(Debug)]
pub struct Output<'bytes> {
    pub bytes: &'bytes mut Vec<u8>,
    pub suffix: Option<String>,
    pub container: Option<String>,
    pub video: Option<Vec<VideoStream>>,
    pub audio: Option<Vec<AudioStream>>,
    pub text: Option<Vec<TextStream>>,
}

#[derive(Debug, Clone, Default)]
pub struct AudioStream {
    pub codec: Option<String>,
    pub channels: Option<u32>,
    pub bitrate: Option<u32>,
}

#[derive(Debug, Clone, Default)]
pub struct VideoStream {
    _not_implemented: PhantomData<Infallible>,
}

#[derive(Debug, Clone, Default)]
pub struct TextStream {
    _not_implemented: PhantomData<Infallible>,
}

#[macro_export]
macro_rules! ffmpeg {
    {
        $(read {
            from $input_path:tt
            $(as $input_format:tt)?
            $(video {
                $(none)?
            })?
        })*

        $(write {
            to $output_path:tt
            $(as $output_format:tt)?
            $(video disabled $output_no_video:vis)?
            $(audio disabled $output_no_audio:vis)?
            $(audio {
                $(as $output_audio_format:tt)?
                $(bps $output_audio_bps:tt)?
                $(channels $output_audio_channels:tt)?
            })?
            $(text disabled $output_no_text:vis)?
            $(text {
                $(as $output_text_format:tt)?
            })?
        })*


    } => {
        {
            let bin = {
                static BIN: ::once_cell::sync::Lazy<::std::path::PathBuf> = ::once_cell::sync::Lazy::new(|| {
                    let name = "ffmpeg";
                    if let Ok(name) = ::which::which(&name) {
                        if let Ok(name) = ::std::fs::canonicalize(&name) {
                            name
                        } else {
                            name
                        }
                    } else {
                        name.into()
                    }
                });
                &*BIN
            };
            let args = [
                $(
                    "-i", $input_path,
                    $(
                        "-f", $input_format,
                    )?
                    $($crate::iff!{$output_no_video then "-nv"},)?
                    $($crate::iff!{$output_no_audio then "-na"},)?
                )*
                $(

                    $output_path,
                )*
            ];
            ::duct::cmd(bin, args)
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! iff {
    ($vis:vis then $($rest:tt)*) => { $($rest)* }
}
