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
use tracing::instrument;
use which::which;

use crate::generic::panic;

fn tempdir() -> Result<impl AsRef<std::path::Path>, std::io::Error> {
    let dir = PathBuf::from("target").join("ffmpegging");
    fs::create_dir_all(&dir);
    tempfile::Builder::new()
        .prefix("")
        .suffix("")
        .tempdir_in(dir)
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

#[instrument]
pub fn wavs_to_opus(wavs: Vec<Vec<u8>>) -> Result<Vec<u8>, panic> {
    let dir = tempdir()?;
    #[cfg(debug_assertions)]
    let dir = Box::leak(Box::new(dir));
    let dir = dir.as_ref();

    let mut args: Vec<OsString> = vec!["-hide_banner".into()];

    for (i, wav) in wavs.iter().enumerate() {
        let path = dir.join(format!("input-{i}.wav"));
        fs::write(&path, wav)?;
        args.extend(["-f", "wav", "-i"].map(Into::into));
        args.push(path.into());
    }

    args.extend(
        [
            "-filter_complex".into(),
            format!("concat=n={n}:v=0:a=1 [output]", n = wavs.len()),
        ]
        .map(Into::into),
    );

    let output = dir.join("output.webm");
    args.extend(["-map", "[output]"].map(Into::into));
    args.extend(["-codec:a", "libopus", "-ab", "32Ki", "-ac", "1"].map(Into::into));
    args.extend(["-codec:v", "webp"].map(Into::into));
    args.extend(["-codec:s", "webvtt"].map(Into::into));
    args.extend(["-f", "webm"].map(Into::into));
    args.push(output.clone().into());

    ffmpeg(args).dir(dir).run()?;

    let output = fs::read(output)?;

    Ok(output)
}
