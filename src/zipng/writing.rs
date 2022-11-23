use {
    crate::{
        alignment::{write_aligned_pad_end, write_aligned_pad_start},
        crc32, panic, WriteAndSeek,
    },
    bstr::ByteSlice,
    std::io::Write,
};

const BLOCK_SIZE: usize = 1024;

/// Creates a zip file from files in the order given, appending to the `prefix`
/// buffer `Vec` (which does not need to be empty), and ending with the
/// given `suffix`,
///
/// Returns the number of bytes written.
pub(crate) fn write_zip(
    mut output: &mut impl WriteAndSeek,
    files: &[(&[u8], &[u8])],
    suffix: &[u8],
) -> Result<usize, panic> {
    if suffix.find(b"PK\x05\x06").is_some() {
        panic!("Zip file suffix must not contain zip file terminator signature PK\\x05\\x06");
    }

    let start = output.stream_position()? as usize;
    let mut files_with_offsets = Vec::with_capacity(files.len());

    for (name, body) in files.iter() {
        let mut header = Vec::new();
        // 0x0000..0x0004: local file header signature
        header.extend_from_slice(b"PK\x03\x04");
        // 0x0004..0x0006: version needed to extract
        header.extend_from_slice(&1_0_u16.to_le_bytes());
        // 0x0006..0x0008: general purpose bit flag
        header.extend_from_slice(&[0x00; 2]);
        // 0x0008..0x000A: compression method
        header.extend_from_slice(&[0x00; 2]);
        // 0x000A..0x000C: modification time
        header.extend_from_slice(b"PK");
        // 0x000C..0x000E: modification date
        header.extend_from_slice(b"PK");
        // 0x000E..0x0012: checksum
        header.extend_from_slice(&crc32(body).to_le_bytes());
        // 0x0012..0x0016: compressed size
        header.extend_from_slice(
            &u32::try_from(body.len())
                .expect("file size larger than 4GiB")
                .to_le_bytes(),
        );
        // 0x0016..0x001A: uncompressed size
        header.extend_from_slice(
            &u32::try_from(body.len())
                .expect("file size larger than 4GiB")
                .to_le_bytes(),
        );
        // 0x001A..0x001E: file name length
        header.extend_from_slice(
            &u16::try_from(name.len())
                .expect("file name larger than 64KiB")
                .to_le_bytes(),
        );
        // 0x001E..0x0022: extra fields length
        header.extend_from_slice(&[0x00; 2]);
        // 0x0022: file name, followed by extra fields (we have none)
        header.extend_from_slice(name);
        let before = output.stream_position()? as usize;
        if !body.is_empty() && name != b"mimetype" {
            write_aligned_pad_start(&mut output, &header, BLOCK_SIZE)?;
            write_aligned_pad_end(&mut output, body, BLOCK_SIZE)?;
        } else {
            output.write_all(&header)?;
            output.write_all(body)?;
        };
        files_with_offsets.push((*name, *body, before));
    }

    let mut central_directory = Vec::new();
    for (name, body, header_offset) in files_with_offsets.iter() {
        let name = name.to_vec();
        let name_length = u16::try_from(name.len()).expect("file name larger than 64KiB");
        let body_length = u32::try_from(body.len()).expect("file size larger than 4GiB");
        let header_offset = u32::try_from(*header_offset).expect("archive larger than 4GiB");
        let crc = crc32(body).to_le_bytes();
        let mut header = Vec::new();
        // 0x0000..0x0004: central file header signature
        header.extend_from_slice(b"PK\x01\x02");
        // 0x0004..0x0006: creator version and platform
        header.extend_from_slice(&1_0_u16.to_le_bytes());
        // 0x0006..0x0008: required version
        header.extend_from_slice(&1_0_u16.to_le_bytes());
        // 0x0008..0x000A: general purpose bit flag
        header.extend_from_slice(&[0x00; 2]);
        // 0x000A..0x000C: compression method
        header.extend_from_slice(&[0x00; 2]);
        // 0x000C..0x000E: modification time
        header.extend_from_slice(b"PK");
        // 0x000E..0x0010: modification date
        header.extend_from_slice(b"PK");
        // 0x0010..0x0014: checksum
        header.extend_from_slice(&crc);
        // 0x0014..0x0018: compressed size
        header.extend_from_slice(&body_length.to_le_bytes());
        // 0x0018..0x001C: uncompressed size
        header.extend_from_slice(&body_length.to_le_bytes());
        // 0x001C..0x001E: file name length
        header.extend_from_slice(&name_length.to_le_bytes());
        // 0x001E..0x0020: extra field length
        header.extend_from_slice(&[0x00; 2]);
        // 0x0020..0x0022: file comment length
        header.extend_from_slice(&[0x00; 2]);
        // 0x0022..0x0024: disk number
        header.extend_from_slice(&[0x00; 2]);
        // 0x0024..0x0026: internal file attributes
        header.extend_from_slice(&[0x00; 2]);
        // 0x0026..0x002A: external file attributes
        header.extend_from_slice(&[0x00; 4]);
        // 0x002A..0x002E: local file header offset from start of archive
        header.extend_from_slice(&header_offset.to_le_bytes());
        // 0x002E..: file name, followed by extra fields and comments (we have none)
        header.extend_from_slice(&name);

        central_directory.extend_from_slice(&header);
    }

    let directory_terminator_len = 22 + suffix.len();

    let before_central_directory = output.stream_position()? as usize;
    output.write_all(&central_directory)?;
    let central_directory_len_without_terminator =
        output.stream_position()? as usize - before_central_directory;

    let _after_central_directory = directory_terminator_len + output.stream_position()? as usize;

    let mut directory_terminator = vec![0; directory_terminator_len];

    let directory_offset =
        u32::try_from(before_central_directory).expect("archive larger than 4GiB");
    let directory_count = u16::try_from(files.len()).expect("more than 64Ki files");
    let directory_length = u32::try_from(central_directory_len_without_terminator).unwrap();
    let suffix_length = u16::try_from(suffix.len()).expect("comment longer than 64KiB");

    // 0x0000..0x0004: archive terminator signature
    directory_terminator.write_all(b"PK\x05\x06").unwrap();
    // 0x0004..0x0006: disk number
    directory_terminator.write_all(&[0x00; 2]).unwrap();
    // 0x0006..0x0008: disk number with central directory
    directory_terminator.write_all(&[0x00; 2]).unwrap();
    // 0x0008..0x000A: directory entries on disk
    directory_terminator
        .write_all(&directory_count.to_le_bytes())
        .unwrap();
    // 0x000A..0x000C: directory entries total
    directory_terminator
        .write_all(&directory_count.to_le_bytes())
        .unwrap();
    // 0x000C..0x0010: central directory byte length
    directory_terminator
        .write_all(&directory_length.to_le_bytes())
        .unwrap();
    // 0x0010..0x0014: central directory offset from start of archive
    directory_terminator
        .write_all(&directory_offset.to_le_bytes())
        .unwrap();
    // 0x0014..: archive comment (suffix) length, then content
    directory_terminator
        .write_all(&suffix_length.to_le_bytes())
        .unwrap();

    output.write_all(&directory_terminator)?;
    output.write_all(suffix)?;
    let end = output.stream_position()? as usize;

    Ok(end - start)
}
