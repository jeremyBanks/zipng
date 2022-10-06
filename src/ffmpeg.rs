#[doc(hidden)]
#[macro_export]
macro_rules! run {
    ($name:ident with $(-$flag:ident $($expr:expr)? ),+) => {
        $crate::run!($name with $(concat!("-", stringify!($flag)) $(,$expr)?),+)
    };
    ($name:ident with $($flag:ident = $expr:expr $(,)?$(;)? )+) => {
        $crate::run!($name with $(concat!("-", stringify!($flag)) ,$expr),+)
    };
    ($name:ident with $($flag:ident: $expr:expr $(,)?$(;)? )+) => {
        $crate::run!($name with $(concat!("-", stringify!($flag)) ,$expr),+)
    };
    ($name:ident with $($expr:expr),*) => {
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

#[macro_export]
macro_rules! ffmpeg(($($tt:tt)*) => ($crate::run!(ffmpeg with $($tt)*)));

#[macro_export]
macro_rules! ffprobe(($($tt:tt)*) => ($crate::run!(ffprobe with $($tt)*)));

#[macro_export]
macro_rules! ffplay(($($tt:tt)*) => ($crate::run!(ffplay with $($tt)*)));
