use {
    crate::{
        byte_buffer, crc32,
        generic::default,
        io::{write_aligned_pad_end, write_aligned_pad_start},
        palettes::{oceanic::BALANCE, RGB_256_COLOR_PALETTE_SIZE},
        panic,
        png::writing::{write_png_header, write_png_palette},
        write_zlib, BitDepth, ColorType, WriteAndSeek, Zip, PNG_CHUNK_PREFIX_SIZE,
        PNG_CHUNK_WRAPPER_SIZE, PNG_HEADER_SIZE, ZIP_FILE_HEADER_EMPTY_SIZE,
    },
    bstr::ByteSlice,
    std::io::Write,
    tracing::warn,
};

/// Serializes a [`Zip`] as a ZIP/PNG polyglot file.
pub fn poc_zipng() -> Result<Vec<u8>, panic> {
    let mut buffer = byte_buffer();

    let zip = Zip {
        files: vec![(
            b"README.md".to_vec(),
            br"\
Blocking waiting for file lock on build directory            
Compiling zipng v0.20221122.0-dev.4 (C:\Users\_\zipng)
Finished dev [unoptimized + debuginfo] target(s) in 3.13s
Running `target\debug\examples\poc.exe`
2022-11-28T20:55:28.068033Z  INFO zipng::png::data: close, time.busy: 222us, time.idle: 17.1us
"
            .to_vec(),
        )],
        ..default()
    };

    write_png_header(&mut buffer, 32, 32, BitDepth::EightBit, ColorType::Indexed)?;
    write_png_palette(&mut buffer, BALANCE)?;

    Ok(buffer.into_inner())
}
