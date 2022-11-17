#![allow(clippy::unusual_byte_groupings, clippy::useless_conversion)]

use std::ops::Not;
use std::ops::Range;

use bstr::BString;
use fiction::panic;
use fiction::png::write_png;
use fiction::png::ColorDepth;
use fiction::png::ColorType;
use fiction::png::PALLETTE_8_BIT_DATA;
use fiction::zip::crc32_zip;
use fiction::zip::zip;
use simd_adler32::adler32;

fn main() -> Result<(), panic> {
    let files: [(&[u8], &[u8]); 4] = [
        (b"mimetype".as_ref(), b"zip/zip".as_ref()),
        (b"README.md".as_ref(), b"welcome".as_ref()),
        (
            b"assets/icon.png".as_ref(),
            include_bytes!("../../icon.png"),
        ),
        (
            b"assets/sqlite_zstd.dll".as_ref(),
            include_bytes!("../../sqlite_zstd.dll"),
        ),
    ];
    let data = zip(files);

    let mut buffer = BString::default();
    let width = 1024;
    write_png(
        &mut buffer,
        &data,
        width,
        (data.len() / width as usize).try_into()?,
        ColorDepth::EightBit,
        ColorType::Index,
        Some(PALLETTE_8_BIT_DATA),
    );

    std::fs::write("target/test.png", buffer)?;

    Ok(())
}
