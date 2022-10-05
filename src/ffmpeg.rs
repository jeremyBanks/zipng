use std::path::PathBuf;

use once_cell::sync::Lazy;
use which::which;

pub static FFMPEG: Lazy<PathBuf> =
    Lazy::new(|| which("ffmpeg").or_else(|_| which("./ffmpeg")).unwrap());

pub static FFPROBE: Lazy<PathBuf> =
    Lazy::new(|| which("ffprobe").or_else(|_| which("./ffprobe")).unwrap());

#[macro_export]
macro_rules! ffmpeg {
    ($(-$flag:ident $($expr:expr)? ),+) => {
        ffmpeg!($(concat!("-", stringify!($flag)) $(,$expr)?),+)
    };
    ($($flag:ident = $expr:expr ),+) => {
        ffmpeg!($(concat!("-", stringify!($flag)) ,$expr),+)
    };
    ($($expr:expr),*) => {
        ::duct::cmd!(&*$crate::ffmpeg::FFMPEG, $($expr),*)
    };
}

#[macro_export]
macro_rules! ffprobe {
    ($(-$flag:ident $($expr:expr)? ),+) => {
        ffprobe!($(concat!("-", stringify!($flag)) $(,$expr)?),+)
    };
    ($($flag:ident = $expr:expr ),+) => {
        ffprobe!($(concat!("-", stringify!($flag)) ,$expr),+)
    };
    ($($expr:expr),*) => {
        ::duct::cmd!(&*$crate::ffprobe::FFPROBE, $($expr),*)
    };
}
