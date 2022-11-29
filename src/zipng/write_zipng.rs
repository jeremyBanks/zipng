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
        PNG_CHUNK_SUFFIX_SIZE, PNG_CHUNK_WRAPPER_SIZE, PNG_HEADER_SIZE, ZIP_FILE_HEADER_EMPTY_SIZE,
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
    let width = 48;
    let height = 96;

    write_png_header(&mut buffer, width + 4, height, color_depth, color_mode)?;
    write_png_palette(&mut buffer, palette)?;

    let mut idat = byte_buffer();
    let mut file_offsets_in_idat = Vec::<usize>::new();

    for file in files.iter() {
        idat.write_all(&[0x00])?; // PNG filter byte, ZIP padding/ignored

        let mut local_file_header = byte_buffer();
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

        file_offsets_in_idat.push(idat.offset());
        idat.write_all(local_file_header.get_ref())?;

        let pixel_data = file.body;

        let mut filtered_data = Vec::new();
        let bits_per_pixel = color_depth.bits_per_sample() * color_mode.samples_per_pixel();
        let bits_per_line = width * bits_per_pixel as u32;
        let bytes_per_line = (bits_per_line + 7) / 8;
        for (i, byte) in pixel_data.iter().enumerate() {
            if i % (bytes_per_line as usize) == 0 {
                // filter byte (uncompressed in PNG) / non-compressed block header (DEFLATE in
                // ZIP)
                filtered_data.push(0); // XXX: for the last block this needs to be marked differently, neh?
                                       // DEFLATE block length (two bytes little endian)
                filtered_data.push(width as u8 + 4);
                filtered_data.push(0);
                // DEFLATE block length bitwise negated (two bytes little endian)
                filtered_data.push(!(width as u8 + 4));
                filtered_data.push(!0);
            }
            filtered_data.push(*byte);
        }

        // blank lines, visual padding
        filtered_data.extend(vec![0x00; width as usize + 5]);
        filtered_data.push(0x00);
        filtered_data.extend(vec![0xFF; width as usize + 4]);
        filtered_data.push(0x00);
        filtered_data.extend(vec![0xFF; width as usize + 4]);
        filtered_data.push(0x00);
        filtered_data.extend(vec![0xFF; width as usize + 4]);

        idat.write_all(&filtered_data)?;
    }

    // empty terminating block
    idat.write_all(&[0x01])?;
    idat.write_all(&[0x00, 0x00])?;
    idat.write_all(&[0xFF, 0xFF])?;

    let offset_before_idat = buffer.offset();
    write_png_body(&mut buffer, idat.get_ref())?;

    let central_directory_offset = buffer.offset() + PNG_CHUNK_PREFIX_SIZE;
    let mut zip_central_directory = byte_buffer();

    for (File { name, body }, offset_in_idat) in files.iter().zip(file_offsets_in_idat) {
        let offset = offset_before_idat + PNG_CHUNK_PREFIX_SIZE + offset_in_idat + 7;
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

    write_non_png_chunk(&mut buffer, zip_central_directory.get_ref())?;

    buffer.write_all(png_footer.get_ref())?;

    Ok(buffer.into_inner())
}
