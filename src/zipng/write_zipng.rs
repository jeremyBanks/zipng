use {
    crate::{
        byte_buffer, crc32,
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
        write_png::{write_png_body, write_png_footer},
        write_zlib, Align, BitDepth, ColorType, Seek, WriteAndSeek, Zip, PNG_CHUNK_PREFIX_SIZE,
        PNG_CHUNK_WRAPPER_SIZE, PNG_HEADER_SIZE, ZIP_FILE_HEADER_EMPTY_SIZE,
    },
    bstr::ByteSlice,
    std::io::Write,
    tracing::{info, trace, warn},
};

pub fn poc_zipng(palette: &[u8]) -> Result<Vec<u8>, panic> {
    let mut buffer = byte_buffer();

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
    let width = 24 * 2;
    let height = 32 * 4 / 2 + 4;

    write_png_header(&mut buffer, width + 4, height, color_depth, color_mode)?;
    write_png_palette(&mut buffer, palette)?;

    let mut idat = byte_buffer();
    let mut file_offsets_in_idat = Vec::<usize>::new();

    for file in files {
        idat.write_all(&[0x00])?; // PNG filter byte, ZIP padding/ignored

        let mut local_file_header = byte_buffer();
        {
            // 0x0000..0x0004: local file header signature
            local_file_header.write(b"PK\x03\x04")?;
            // 0x0004..0x0006: version needed to extract
            local_file_header.write(&1_0_u16.to_le_bytes())?;
            // 0x0006..0x0008: general purpose bit flag
            local_file_header.write(&[0x00; 2])?;
            // 0x0008..0x000A: compression method
            local_file_header.write(&[0x00; 2])?;
            // 0x000A..0x000C: modification time
            local_file_header.write(b"PK")?;
            // 0x000C..0x000E: modification date
            local_file_header.write(b"PK")?;
            // 0x000E..0x0012: checksum
            local_file_header.write(&crc32(file.body).to_le_bytes())?;
            // 0x0012..0x0016: compressed size
            local_file_header.write(
                &u32::try_from(file.body.len())
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
        info!(padding_required = padding_required);
        for i in 0..padding_required {
            static padding_bytes: [u8; 4] = [0x00, 0x00, 0xFF, 0xFF];
            idat.write_all(&[padding_bytes[i % padding_bytes.len()]])?;
        }
        file_offsets_in_idat.push(idat.offset());
        idat.write_all(local_file_header.get_ref())?;

        let pixel_data = file.body;

        // The minimum size for a local ZIP header is 34 bytes (with NO filename).
        // That fits, but barely.

        let mut filtered_data = Vec::new();
        let bits_per_pixel = color_depth.bits_per_sample() * color_mode.samples_per_pixel();
        let bits_per_line = width * bits_per_pixel as u32;
        let bytes_per_line = (bits_per_line + 7) / 8;
        for (i, byte) in pixel_data.iter().enumerate() {
            if i % (bytes_per_line as usize) == 0 {
                // filter byte (uncompressed in PNG) / non-compressed block header (DEFLATE in
                // ZIP)
                filtered_data.push(0);
                // DEFLATE block length (two bytes little endian)
                filtered_data.push(width as u8 + 4);
                filtered_data.push(0);
                // DEFLATE block length bitwise negated (two bytes little endian)
                filtered_data.push(!(width as u8 + 4));
                filtered_data.push(!0);
            }
            filtered_data.push(*byte);
        }
        idat.write_all(&filtered_data)?;
    }
    write_png_body(&mut buffer, idat.get_ref())?;

    let zip_file_header = byte_buffer();
    write_non_png_chunk(&mut buffer, zip_file_header.get_ref())?;

    let mut suffix = byte_buffer();
    write_png_footer(&mut suffix)?;

    Ok(buffer.into_inner())
}
