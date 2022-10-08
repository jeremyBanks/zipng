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

pub fn wavs_to_opus(wavs: Vec<Vec<u8>>) -> Result<Vec<u8>, eyre::Report> {
    let dir = tempdir()?;
    // let dir = Box::leak(Box::new(dir));
    let dir = dir.as_ref();

    let mut args = Vec::<OsString>::new();
    let output = dir.join("out.opus.webm");

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
            "-map".into(),
            "[output]".into(),
        ]
        .map(Into::into),
    );

    args.extend(["-acodec", "libopus", "-ab", "24Ki", "-ac", "1"].map(Into::into));
    args.extend(["-vcodec", "webp"].map(Into::into));
    args.extend(["-scodec", "webvtt"].map(Into::into));
    args.extend(["-f", "webm"].map(Into::into));
    args.push(output.clone().into());

    args.push("-hide_banner".into());

    ffmpeg(args).dir(dir).run()?;

    Ok(fs::read(output)?)
}
