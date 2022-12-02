use {
    crate::{
        crc32, output_buffer, panic, write_zlib, BitDepth, ColorType, InputWrite,
        PNG_HEADER_SIZE,
    },
    std::{
        io::{Cursor, Write},
    },
    tracing::warn,
};

pub fn write_png_header(
    buffer: &mut impl InputWrite,
    width: u32,
    height: u32,
    color_depth: BitDepth,
    color_mode: ColorType,
) -> Result<usize, panic> {
    let before = buffer.offset();
    if before != 0 {
        warn!("PNG header is being written at nonzero stream offset: {before}");
    }

    // 0x0000..0x0008: PNG signature
    buffer.write_all(b"\x89PNG\r\n\x1A\n")?;
    // 0x0008..0x0010:     IHDR chunk prefix (length and type)
    write_png_chunk(buffer, b"IHDR", &{
        let mut data = output_buffer();

        // 0x0010..0x0014: pixel width
        data.write_all(&width.to_be_bytes())?;
        // 0x0004..0x0018: pixel height
        data.write_all(&height.to_be_bytes())?;
        // 0x0018:         color bit depth
        data.write_all(&u8::from(color_depth).to_be_bytes())?;
        // 0x0019:         color type: grayscale
        data.write_all(&u8::from(color_mode).to_be_bytes())?;
        // 0x001A:         compression method: deflate
        data.write_all(&0_u8.to_be_bytes())?;
        // 0x001B:         filter method: basic
        data.write_all(&0_u8.to_be_bytes())?;
        // 0x001C:         interlace method: none
        data.write_all(&0_u8.to_be_bytes())?;

        data.into_bytes()
        // 0x001D..0x0025: IHDR chunk suffix (checksum)
    })?;

    let after = buffer.offset();

    assert_eq!(after - before, PNG_HEADER_SIZE);

    Ok(PNG_HEADER_SIZE)
}

pub fn write_png_palette(buffer: &mut impl InputWrite, palette: &[u8]) -> Result<usize, panic> {
    write_png_chunk(buffer, b"PLTE", palette)
}

pub fn write_png_body(buffer: &mut impl InputWrite, data: &[u8]) -> Result<usize, panic> {
    let mut deflated = Vec::new();
    write_zlib(&mut Cursor::new(&mut deflated), data)?;
    write_png_chunk(buffer, b"IDAT", &deflated)
}

pub fn write_non_png_chunk(buffer: &mut impl InputWrite, data: &[u8]) -> Result<usize, panic> {
    write_png_chunk(buffer, b"pkPK", data)
}

pub fn write_png_footer(buffer: &mut impl InputWrite) -> Result<usize, panic> {
    write_png_chunk(buffer, b"IEND", b"")
}

pub fn write_png_chunk(
    buffer: &mut impl InputWrite,
    chunk_type: &[u8; 4],
    data: &[u8],
) -> Result<usize, panic> {
    let before = buffer.offset();

    buffer.write_all(
        &u32::try_from(data.len())
            .expect("png chunk larger than 2GiB")
            .to_be_bytes(),
    )?;
    buffer.write_all(chunk_type)?;
    buffer.write_all(data)?;
    buffer.write_all(
        &crc32(
            &[chunk_type.as_slice(), data]
                .into_iter()
                .flatten()
                .copied()
                .collect::<Vec<_>>(),
        )
        .to_be_bytes(),
    )?;

    let after = buffer.offset();
    Ok(after - before)
}

pub fn write_png(
    buffer: &mut impl InputWrite,
    pixel_data: &[u8],
    width: u32,
    height: u32,
    bit_depth: BitDepth,
    color_mode: ColorType,
    palette: Option<&[u8]>,
) -> Result<(), panic> {
    write_png_header(buffer, width, height, bit_depth, color_mode)?;
    if let Some(palette) = palette {
        write_png_palette(buffer, palette)?;
    }
    // We need to insert a 0x00 byte at the start of every line (every `width`
    // bytes) to indicate that the line is not filtered.
    let mut filtered_data = Vec::new();

    let bits_per_pixel = bit_depth.bits_per_sample() * color_mode.samples_per_pixel();
    let bits_per_line = width * bits_per_pixel as u32;
    let bytes_per_line = (bits_per_line + 7) / 8;

    for (i, byte) in pixel_data.iter().enumerate() {
        if i % (bytes_per_line as usize) == 0 {
            filtered_data.push(0x00);
        }
        filtered_data.push(*byte);
    }
    write_png_body(buffer, &filtered_data)?;
    write_png_footer(buffer)?;
    Ok(())
}
