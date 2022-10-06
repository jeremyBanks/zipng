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
