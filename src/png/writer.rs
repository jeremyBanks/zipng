use {
    crate::{adler32, crc32, BitDepth, ColorType},
    std::ops::{Not, Range},
};

pub fn write_png_header(
    buffer: &mut Vec<u8>,
    width: u32,
    height: u32,
    color_depth: BitDepth,
    color_mode: ColorType,
) -> Range<usize> {
    let before = buffer.len();

    buffer.extend_from_slice(b"\x89PNG\r\n\x1A\n");
    write_png_chunk(buffer, b"IHDR", &{
        let mut data = Vec::new();
        // pixel width
        data.extend_from_slice(&width.to_be_bytes());
        // pixel height
        data.extend_from_slice(&height.to_be_bytes());
        // color bit depth
        data.extend_from_slice(&u8::from(color_depth).to_be_bytes());
        // color type: grayscale
        data.extend_from_slice(&u8::from(color_mode).to_be_bytes());
        // compression method: deflate
        data.extend_from_slice(&0_u8.to_be_bytes());
        // filter method: basic
        data.extend_from_slice(&0_u8.to_be_bytes());
        // interlace method: none
        data.extend_from_slice(&0_u8.to_be_bytes());

        data
    });

    let after = buffer.len();
    before..after
}

pub fn write_png_palette(buffer: &mut Vec<u8>, palette: &[u8]) -> Range<usize> {
    write_png_chunk(buffer, b"PLTE", palette)
}

pub fn write_png_body(buffer: &mut Vec<u8>, data: &[u8]) -> Range<usize> {
    let mut deflated = Vec::new();
    write_non_deflated(&mut deflated, data);
    write_png_chunk(buffer, b"IDAT", &deflated)
}

pub fn write_non_deflated(buffer: &mut Vec<u8>, data: &[u8]) -> Range<usize> {
    let chunks = data.chunks(0xFFFF);

    // zlib compression mode: deflate with 32KiB windows
    let cmf = 0b_0111_1000;
    buffer.push(cmf);
    // zlib flag bits: no preset dictionary, compression level 0
    let mut flg: u8 = 0b0000_0000;
    // zlib flag and check bits
    flg |= 0b1_1111 - ((((cmf as u16) << 8) | (flg as u16)) % 0b1_1111) as u8;
    buffer.push(flg);

    let mut before = None;
    let mut after = None;

    let count = chunks.len();
    for (index, chunk) in chunks.enumerate() {
        // deflate flag bits
        let is_last_chunk = index + 1 >= count;
        buffer.push(is_last_chunk.into());
        // deflate block length
        buffer.extend_from_slice(&u16::try_from(chunk.len()).unwrap().to_le_bytes());
        // deflate block length check complement
        buffer.extend_from_slice(&u16::try_from(chunk.len()).unwrap().not().to_le_bytes());

        // Is there any way I can massage this into looking like a PNG
        // filter prefix? We can't get rid of the lengths, and they're constant, but I
        // guess we can accept that as a border. hmm... if at least one of their
        // bytes is zero (yes), it will be possible to treat the middle byte as the
        // filter and have the border on both sides.

        before.get_or_insert(buffer.len());
        buffer.extend_from_slice(chunk);
        after = Some(buffer.len());
    }

    // adler-32 checksum of the uncompressed data
    buffer.extend_from_slice(&adler32(data).to_le_bytes());

    let after = after.unwrap_or(buffer.len());
    let before = before.unwrap_or(after);

    before..after
}

pub fn write_non_png_chunk(buffer: &mut Vec<u8>, data: &[u8]) -> Range<usize> {
    write_png_chunk(buffer, b"pkPK", data)
}

pub fn write_png_footer(buffer: &mut Vec<u8>) -> Range<usize> {
    write_png_chunk(buffer, b"IEND", b"")
}

pub fn write_png_chunk(buffer: &mut Vec<u8>, chunk_type: &[u8; 4], data: &[u8]) -> Range<usize> {
    let before = buffer.len();

    buffer.extend_from_slice(
        &u32::try_from(data.len())
            .expect("png chunk larger than 2GiB")
            .to_be_bytes(),
    );
    buffer.extend_from_slice(chunk_type);
    buffer.extend_from_slice(data);
    buffer.extend_from_slice(
        &crc32(
            &[chunk_type.as_slice(), data]
                .into_iter()
                .flatten()
                .copied()
                .collect::<Vec<_>>(),
        )
        .to_be_bytes(),
    );

    let after = buffer.len();
    before..after
}

pub fn write_png(
    buffer: &mut Vec<u8>,
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
