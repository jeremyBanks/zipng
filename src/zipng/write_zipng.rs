use {
    crate::{
        adler32, byte_buffer, crc32,
        generic::default,
        io::{write_aligned_pad_end, write_aligned_pad_start},
        palettes::{
            oceanic::BALANCE,
            singles::{CIVIDIS, TURBO},
            viridis::MAGMA,
            RGB_256_COLOR_PALETTE_SIZE,
        },
        panic,
        png::write_png::{write_non_png_chunk, write_png_header, write_png_palette},
        write_aligned,
        write_png::{write_png_body, write_png_chunk, write_png_footer},
        write_zlib, Align, BitDepth, ColorType, Offset, WriteAndSeek, Zip, PNG_CHUNK_PREFIX_SIZE,
        PNG_CHUNK_SUFFIX_SIZE, PNG_CHUNK_WRAPPER_SIZE, PNG_HEADER_SIZE, ZIP_FILE_HEADER_EMPTY_SIZE,
    },
    bstr::ByteSlice,
    std::{borrow::Cow, io::Write},
    tracing::{error, info, trace, warn},
};

pub fn poc_zipng(palette: &[u8]) -> Result<Vec<u8>, panic> {
    let mut output = output_buffer();

    // For this proof-of-concept, let's use two files.
    struct File {
        name: &'static [u8],
        body: &'static [u8],
    }
    let files = [
        File {
            name: b"BALANCE.plte",
            body: BALANCE,
        },
        File {
            name: b"TURBO.plte",
            body: TURBO,
        },
        File {
            name: b"CIVIDIS.plte",
            body: CIVIDIS,
        },
        File {
            name: b"MAGMA.plte",
            body: MAGMA,
        },
    ];

    let color_depth = BitDepth::EightBit;
    let color_mode = ColorType::Indexed;
    let width = 48;
    let height = 96;

    output.png_tag_start("PNG file");

    output.png_tag_start("IHDR header");
    write_png_header(&mut output, width + 4, height, color_depth, color_mode)?;
    output.png_tag_end("IHDR header");

    output.png_tag_start("PLTE palette");
    write_png_palette(&mut output, palette)?;
    output.png_tag_end("PLTE palette");

    let mut idat = byte_buffer();
    let mut file_offsets_in_idat = Vec::<usize>::new();

    for file in files.iter() {
        let mut local_file_header = byte_buffer();
        local_file_header.write_all(&[0x00])?; // PNG filter byte, ZIP padding/ignored

        {
            // 0x0000..0x0004: local file header signature
            local_file_header.write(b"PK\x03\x04")?;
            // 0x0004..0x0006: version needed to extract
            local_file_header.write(&2_0_u16.to_le_bytes())?;
            // 0x0006..0x0008: general purpose bit flag
            local_file_header.write(&[0x00; 0x00])?;
            // 0x0008..0x000A: compression method -- DEFLATE
            local_file_header.write(&0x08_u16.to_le_bytes())?;
            // 0x000A..0x000C: modification time
            local_file_header.write(b"PK")?;
            // 0x000C..0x000E: modification date
            local_file_header.write(b"PK")?;
            // 0x000E..0x0012: checksum
            local_file_header.write(&crc32(file.body).to_le_bytes())?;
            // 0x0012..0x0016: compressed size
            local_file_header.write(
                &u32::try_from((file.body.len()) / (width as usize) * ((width as usize) + 5))
                    .expect("file size larger than 4GiB")
                    .to_le_bytes(),
            )?;
            // 0x0016..0x001A: uncompressed size
            local_file_header.write(
                &u32::try_from(file.body.len())
                    .expect("file size larger than 4GiB")
                    .to_le_bytes(),
            )?;
            // 0x001A..0x001E: file name length
            local_file_header.write(
                &u16::try_from(file.name.len())
                    .expect("file name larger than 64KiB")
                    .to_le_bytes(),
            )?;
            // 0x001E..0x0022: extra fields length
            local_file_header.write(&[0x00; 2])?;
            // 0x0022: file name, followed by extra fields (we have none)
            local_file_header.write(file.name)?;
        }

        assert!(local_file_header.offset() < width as usize + 4);
        let padding_required = width as usize + 4 - local_file_header.offset();
        for i in 0..padding_required {
            static padding_bytes: [u8; 4] = [0x00, 0x00, 0x00, 0x00];
            idat.write_all(&[padding_bytes[i % padding_bytes.len()]])?;
        }

        file_offsets_in_idat.push(idat.offset() + 1);
        idat.write_all(local_file_header.get_ref())?;

        let pixel_data = file.body;

        let mut data_with_chunk_headers = Vec::new();

        // zlib compression mode: deflate with 32KiB windows
        let cmf = 0b_0111_1000;
        idat.write_all(&[cmf])?;
        // zlib flag bits: no preset dictionary, compression level 0
        let mut flg: u8 = 0b0000_0000;
        // zlib flag and check bits
        flg |= 0b1_1111 - ((((cmf as u16) << 8) | (flg as u16)) % 0b1_1111) as u8;
        data_with_chunk_headers.write_all(&[flg])?;

        let bits_per_pixel = color_depth.bits_per_sample() * color_mode.samples_per_pixel();
        let bits_per_line = width * bits_per_pixel as u32;
        let bytes_per_line = (bits_per_line + 7) / 8;
        for (i, byte) in pixel_data.iter().enumerate() {
            if i % (bytes_per_line as usize) == 0 {
                // filter byte (uncompressed in PNG) / non-compressed block header (DEFLATE in
                // ZIP)
                data_with_chunk_headers.push(0); // XXX: for the last block this needs to be marked differently, neh?
                                                 // DEFLATE block length (two bytes little endian)
                data_with_chunk_headers.push(width as u8 + 4);
                data_with_chunk_headers.push(0);
                // DEFLATE block length bitwise negated (two bytes little endian)
                data_with_chunk_headers.push(!(width as u8 + 4));
                data_with_chunk_headers.push(!0);
            }
            data_with_chunk_headers.push(*byte);
        }

        // blank lines, visual padding
        data_with_chunk_headers.push(0x00);
        data_with_chunk_headers.extend(vec![0x00; width as usize + 4]);
        data_with_chunk_headers.push(0x00);
        data_with_chunk_headers.extend(vec![0xFF; width as usize + 4]);
        data_with_chunk_headers.push(0x00);
        data_with_chunk_headers.extend(vec![0xFF; width as usize + 4]);
        data_with_chunk_headers.push(0x01); // last block
        data_with_chunk_headers.extend(vec![0xFF; width as usize + 4]);

        idat.write_all(&data_with_chunk_headers)?;
    }

    // empty terminating block
    idat.write_all(&[0x01])?;
    idat.write_all(&[0x00, 0x00])?;
    idat.write_all(&[0xFF, 0xFF])?;

    let offset_before_idat = output.offset();

    let data: &[u8] = idat.get_ref();

    // this is wrong because we it's looking at the compressed data, not the
    // uncompressed data
    idat.write_all(&adler32(idat.get_ref()).to_le_bytes())?;

    output.png_tag_start("IDAT data");
    write_png_chunk(&mut output, b"IDAT", idat.get_ref())?;
    output.png_tag_end("IDAT data");

    let central_directory_offset = output.offset() + PNG_CHUNK_PREFIX_SIZE;
    let mut zip_central_directory = byte_buffer();

    for (File { name, body }, offset_in_idat) in files.iter().zip(file_offsets_in_idat) {
        let offset = offset_before_idat + PNG_CHUNK_PREFIX_SIZE + offset_in_idat;
        let mut file_entry = byte_buffer();

        let name = name.to_vec();
        let name_length = u16::try_from(name.len()).expect("file name larger than 64KiB");
        let body_length = u32::try_from(body.len()).expect("file size larger than 4GiB");
        let header_offset = u32::try_from(offset).expect("archive larger than 4GiB");
        let crc = crc32(body).to_le_bytes();
        // 0x0000..0x0004: central file header signature
        file_entry.write_all(b"PK\x01\x02")?;
        // 0x0004..0x0006: creator version and platform
        file_entry.write_all(&2_0_u16.to_le_bytes())?;
        // 0x0006..0x0008: required version
        file_entry.write_all(&2_0_u16.to_le_bytes())?;
        // 0x0008..0x000A: general purpose bit flag
        file_entry.write_all(&[0x00; 2])?;
        // 0x000A..0x000C: compression method
        file_entry.write_all(&0x08_u16.to_le_bytes())?;
        // 0x000C..0x000E: modification time
        file_entry.write_all(b"PK")?;
        // 0x000E..0x0010: modification date
        file_entry.write_all(b"PK")?;
        // 0x0010..0x0014: checksum
        file_entry.write_all(&crc)?;
        // 0x0014..0x0018: compressed size
        // XXX: I need to record the actual value for this
        file_entry.write_all(
            &u32::try_from((body_length) / (width) * ((width) + 5))
                .expect("file size larger than 4GiB")
                .to_le_bytes(),
        )?;
        // 0x0018..0x001C: uncompressed size
        file_entry.write_all(&body_length.to_le_bytes())?;
        // 0x001C..0x001E: file name length
        file_entry.write_all(&name_length.to_le_bytes())?;
        // 0x001E..0x0020: extra field length
        file_entry.write_all(&[0x00; 2])?;
        // 0x0020..0x0022: file comment length
        file_entry.write_all(&[0x00; 2])?;
        // 0x0022..0x0024: disk number
        file_entry.write_all(&[0x00; 2])?;
        // 0x0024..0x0026: internal file attributes
        file_entry.write_all(&[0x00; 2])?;
        // 0x0026..0x002A: external file attributes
        file_entry.write_all(&[0x00; 4])?;
        // 0x002A..0x002E: local file header offset from start of archive
        file_entry.write_all(&header_offset.to_le_bytes())?;
        // 0x002E..: file name, followed by extra fields and comments (we have none)
        file_entry.write_all(&name)?;

        zip_central_directory.write_all(file_entry.get_ref())?;
    }

    let mut png_footer = byte_buffer();
    write_png_footer(&mut png_footer)?;

    let mut directory_terminator = byte_buffer();
    // 0x0000..0x0004: archive terminator signature
    directory_terminator.write_all(b"PK\x05\x06").unwrap();
    // 0x0004..0x0006: disk number
    directory_terminator.write_all(&[0x00; 2]).unwrap();
    // 0x0006..0x0008: disk number with central directory
    directory_terminator.write_all(&[0x00; 2]).unwrap();
    // 0x0008..0x000A: directory entries on disk
    directory_terminator
        .write_all(&u16::try_from(files.len()).unwrap().to_le_bytes())
        .unwrap();
    // 0x000A..0x000C: directory entries total
    directory_terminator
        .write_all(&u16::try_from(files.len()).unwrap().to_le_bytes())
        .unwrap();
    // 0x000C..0x0010: central directory byte length
    directory_terminator
        .write_all(
            &u32::try_from(zip_central_directory.len())
                .unwrap()
                .to_le_bytes(),
        )
        .unwrap();
    // 0x0010..0x0014: central directory offset from start of archive
    directory_terminator
        .write_all(
            &u32::try_from(central_directory_offset)
                .unwrap()
                .to_le_bytes(),
        )
        .unwrap();
    // 0x0014..: archive comment (suffix) length
    directory_terminator
        .write_all(
            &u16::try_from(png_footer.len() + PNG_CHUNK_SUFFIX_SIZE)
                .unwrap()
                .to_le_bytes(),
        )
        .unwrap();

    zip_central_directory.write_all(directory_terminator.get_ref())?;

    write_non_png_chunk(&mut output, zip_central_directory.get_ref())?;

    output.write_all(png_footer.get_ref())?;

    Ok(output.into_inner())
}

type CowStr = Cow<'static, str>;

pub fn output_buffer() -> OutputBuffer {
    OutputBuffer::new()
}

#[derive(Debug, Default, Clone)]
pub struct OutputBuffer {
    bytes: Vec<u8>,
    png_tags_by_byte: Vec<Vec<CowStr>>,
    zip_tags_by_byte: Vec<Vec<CowStr>>,
    png_tag_stack: Vec<CowStr>,
    zip_tag_stack: Vec<CowStr>,
    png_tags_for_next: Vec<CowStr>,
    zip_tags_for_next: Vec<CowStr>,
}

impl OutputBuffer {
    pub fn new() -> Self {
        default()
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.bytes
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn push(&mut self, byte: u8) {
        let mut png_tags = self.png_tag_stack.clone();
        png_tags.extend(self.png_tags_for_next.drain(..));
        let mut zip_tags = self.png_tag_stack.clone();
        zip_tags.extend(self.png_tags_for_next.drain(..));
        self.bytes.push(byte);
        self.png_tags_by_byte.push(png_tags);
        self.zip_tags_by_byte.push(zip_tags);

        let fresh_png_tags = self.png_tags_by_byte.len() < 2
            || self.png_tags_by_byte[self.png_tags_by_byte.len() - 2]
                != self.png_tags_by_byte[self.png_tags_by_byte.len() - 1];
        let fresh_zip_tags = self.zip_tags_by_byte.len() < 2
            || self.zip_tags_by_byte[self.zip_tags_by_byte.len() - 2]
                != self.zip_tags_by_byte[self.zip_tags_by_byte.len() - 1];

        if fresh_png_tags || fresh_zip_tags {
            trace!(
                "\n[0x{index:04X}] = 0x{byte:02X} with\n  PNG tags: {png_tags:?}\n  ZIP tags: \
                 {zip_tags:?}",
                index = self.bytes.len() - 1,
                byte = byte,
                png_tags = self.png_tags_by_byte.last().unwrap(),
                zip_tags = self.zip_tags_by_byte.last().unwrap()
            );
        }
    }

    pub fn png_tag_start(&mut self, tag: impl Into<CowStr>) {
        self.png_tag_stack.push(tag.into());
    }

    pub fn png_tag_end(&mut self, tag: impl Into<CowStr>) {
        let tag = tag.into();
        let popped = self.png_tag_stack.pop();
        if popped.as_ref() != Some(&tag) {
            panic!("expected end of tag {tag:?}, but got end of {popped:?}");
        }
    }

    pub fn png_tag(&mut self, tag: impl Into<CowStr>) {
        self.png_tags_for_next.push(tag.into());
    }

    pub fn zip_tag_start(&mut self, tag: impl Into<CowStr>) {
        self.zip_tag_stack.push(tag.into());
    }

    pub fn zip_tag_end(&mut self, tag: impl Into<CowStr>) {
        let tag = tag.into();
        let popped = self.zip_tag_stack.pop();
        if popped.as_ref() != Some(&tag) {
            panic!("expected end of tag {tag:?}, but got end of {popped:?}");
        }
    }

    pub fn zip_tag(&mut self, tag: impl Into<CowStr>) {
        self.zip_tags_for_next.push(tag.into());
    }
}

impl Write for OutputBuffer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        for byte in buf {
            self.push(*byte);
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Offset for OutputBuffer {
    fn offset(&mut self) -> usize {
        self.len()
    }

    fn len(&mut self) -> usize {
        self.bytes.len()
    }
}
