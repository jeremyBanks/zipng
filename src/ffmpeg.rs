use std::convert::Infallible;
use std::ffi::OsString;
use std::fs;
use std::io::Read;
use std::io::Write;
use std::marker::PhantomData;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use once_cell::sync::Lazy;
use which::which;

fn tempdir() -> Result<impl AsRef<std::path::Path>, std::io::Error> {
    tempfile::Builder::new()
        .prefix("")
        .suffix("")
        .tempdir_in(PathBuf::from("target").join("ffmpegging"))
}

fn ffmpeg<Args>(args: Args) -> duct::Expression
where
    Args: IntoIterator,
    Args::Item: Into<OsString>,
{
    static FFMPEG: Lazy<PathBuf> = Lazy::new(|| {
        let name = "ffmpeg";
        if let Ok(name) = which(&name) {
            if let Ok(name) = name.canonicalize() {
                name
            } else {
                name
            }
        } else {
            name.into()
        }
    });
    duct::cmd(&*FFMPEG, args)
}

pub fn wavs_to_opus(wavs: Vec<Vec<u8>>) -> Result<Vec<u8>, eyre::Report> {
    let dir = tempdir()?;
    let dir = Box::leak(Box::new(dir));
    let dir = dir.as_ref();

    let mut args = Vec::<OsString>::new();
    let output = dir.join("out.opus.mka");
    let mut filter_inputs = String::new();

    for (i, wav) in wavs.iter().enumerate() {
        let path = dir.join(format!("input-{i}.wav"));
        fs::write(&path, wav)?;
        args.extend(["-f".into(), "wav".into(), "-i".into(), path.into()]);
        filter_inputs += &format!("[{i}:0] ");
    }

    args.extend(
        [
            "-filter_complex".into(),
            filter_inputs + &format!("concat=n={n}:v=0:a=1 [a]", n = wavs.len()),
            "-map".into(),
            "[a]".into(),
        ]
        .map(Into::into),
    );

    args.extend(["-acodec", "libopus", "-frame_duration", "60", "-ab", "24Ki"].map(Into::into));
    args.extend(["-vcodec", "png"].map(Into::into));
    args.extend(["-scodec", "webvtt"].map(Into::into));
    args.extend(["-f", "webm"].map(Into::into));
    args.push(output.clone().into());

    args.push("-hide_banner".into());

    ffmpeg(args).dir(dir).run()?;

    Ok(fs::read(output)?)
}
