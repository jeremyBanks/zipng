use {
    crate::{adler32, crc32, panic, BitDepth, ColorType, WriteAndSeek},
    std::{
        io::{Cursor, Write},
        ops::{Not, Range},
    },
    tracing::warn,
};

pub const PNG_HEADER_SIZE: usize = 33;
pub fn write_png_header(
    buffer: &mut impl WriteAndSeek,
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
        let mut data = Cursor::new(Vec::new());

        // 0x0010..0x0014: pixel width
        data.write_all(&width.to_be_bytes());
        // 0x0004..0x0018: pixel height
        data.write_all(&height.to_be_bytes());
        // 0x0018:         color bit depth
        data.write_all(&u8::from(color_depth).to_be_bytes());
        // 0x0019:         color type: grayscale
        data.write_all(&u8::from(color_mode).to_be_bytes());
        // 0x001A:         compression method: deflate
        data.write_all(&0_u8.to_be_bytes());
        // 0x001B:         filter method: basic
        data.write_all(&0_u8.to_be_bytes());
        // 0x001C:         interlace method: none
        data.write_all(&0_u8.to_be_bytes());

        data.into_inner()
        // 0x001D..0x0025: IHDR chunk suffix (checksum)
    });

    let after = buffer.offset();

    assert_eq!(after - before, PNG_HEADER_SIZE);

    Ok(PNG_HEADER_SIZE)
}

pub fn write_png_palette(buffer: &mut impl WriteAndSeek, palette: &[u8]) -> Result<usize, panic> {
    write_png_chunk(buffer, b"PLTE", palette)
}

pub fn write_png_body(buffer: &mut impl WriteAndSeek, data: &[u8]) -> Result<usize, panic> {
    let mut deflated = Vec::new();
    write_non_deflated(&mut Cursor::new(&mut deflated), data);
    write_png_chunk(buffer, b"IDAT", &deflated)
}

pub fn write_non_deflated(buffer: &mut impl WriteAndSeek, data: &[u8]) -> Result<usize, panic> {
    let chunks = data.chunks(0xFFFF);

    // zlib compression mode: deflate with 32KiB windows
    let cmf = 0b_0111_1000;
    buffer.write_all(&[cmf])?;
    // zlib flag bits: no preset dictionary, compression level 0
    let mut flg: u8 = 0b0000_0000;
    // zlib flag and check bits
    flg |= 0b1_1111 - ((((cmf as u16) << 8) | (flg as u16)) % 0b1_1111) as u8;
    buffer.write_all(&[flg])?;

    let mut before = None;
    let mut after = None;

    let count = chunks.len();
    for (index, chunk) in chunks.enumerate() {
        // deflate flag bits
        let is_last_chunk = index + 1 >= count;
        buffer.write_all(&[is_last_chunk.into()])?;
        // deflate block length
        buffer.write_all(&u16::try_from(chunk.len()).unwrap().to_le_bytes());
        // deflate block length check complement
        buffer.write_all(&u16::try_from(chunk.len()).unwrap().not().to_le_bytes());

        // Is there any way I can massage this into looking like a PNG
        // filter prefix? We can't get rid of the lengths, and they're constant, but I
        // guess we can accept that as a border. hmm... if at least one of their
        // bytes is zero (yes), it will be possible to treat the middle byte as the
        // filter and have the border on both sides.

        before.get_or_insert(buffer.offset());
        buffer.write_all(chunk);
        after = Some(buffer.offset());
    }

    // adler-32 checksum of the uncompressed data
    buffer.write_all(&adler32(data).to_le_bytes());

    let after = after.unwrap_or(buffer.offset());
    let before = before.unwrap_or(after);

    Ok(before - after)
}

pub fn write_non_png_chunk(buffer: &mut impl WriteAndSeek, data: &[u8]) -> Result<usize, panic> {
    write_png_chunk(buffer, b"pkPK", data)
}

pub fn write_png_footer(buffer: &mut impl WriteAndSeek) -> Result<usize, panic> {
    write_png_chunk(buffer, b"IEND", b"")
}

pub fn write_png_chunk(
    buffer: &mut impl WriteAndSeek,
    chunk_type: &[u8; 4],
    data: &[u8],
) -> Result<usize, panic> {
    let before = buffer.offset();

    buffer.write_all(
        &u32::try_from(data.len())
            .expect("png chunk larger than 2GiB")
            .to_be_bytes(),
    );
    buffer.write_all(chunk_type);
    buffer.write_all(data);
    buffer.write_all(
        &crc32(
            &[chunk_type.as_slice(), data]
                .into_iter()
                .flatten()
                .copied()
                .collect::<Vec<_>>(),
        )
        .to_be_bytes(),
    );

    let after = buffer.offset();
    Ok(before - after)
}

pub fn write_png(
    buffer: &mut impl WriteAndSeek,
    pixel_data: &[u8],
    width: u32,
    height: u32,
    bit_depth: BitDepth,
    color_mode: ColorType,
    palette: Option<&[u8]>,
) {
    write_png_header(buffer, width, height, bit_depth, color_mode);
    if let Some(palette) = palette {
        write_png_palette(buffer, palette);
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
    write_png_body(buffer, &filtered_data);
    write_png_footer(buffer);
}
