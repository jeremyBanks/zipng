use std::convert::Infallible;
use std::ffi::OsString;
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
        .prefix("ffmpegging")
        .tempdir_in("target")
}

fn ffmpeg<Args>(args: Args) -> duct::Expression
where
    Args: IntoIterator,
    Args::Item: Into<OsString>,
{
    static PATH: Lazy<PathBuf> = Lazy::new(|| {
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
    duct::cmd(&*PATH, args)
}

pub fn wavs_to_opus(wavs: Vec<Vec<u8>>) -> Result<Vec<u8>, eyre::Report> {
    let dir = tempdir()?;
    let dir = dir.as_ref();

    let mut args = Vec::<OsString>::new();
    let opus = dir.join("out.opus.mka");
    let mut filter_inputs = String::new();

    for (i, wav) in wavs.iter().enumerate() {
        let path = dir.join(format!("{}.wav", i));
        std::fs::write(&path, wav)?;
        args.extend(["-i".into(), path.into()]);
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

    args.extend(["-ac", "opus", "-ab", "24Ki"].map(Into::into));
    args.extend(["-vn", "-sn"].map(Into::into));
    args.push(opus.clone().into());

    ffmpeg(args).dir(dir).run()?;

    Ok(std::fs::read(opus)?)
}
