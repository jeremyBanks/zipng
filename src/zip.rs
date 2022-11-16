//! Zip container encoder.
//!
//! Produces a canonical non-compressed zip file with the given contents.
//! Files are aligned to 1024-byte blocks.
use std::collections::BTreeMap;
use std::io::Write;
use std::ops::Range;

use crc::Crc;
use crc::CRC_32_ISO_HDLC;

const BLOCK_SIZE: usize = 1024;

pub fn zip(files: BTreeMap<&[u8], &[u8]>) -> Vec<u8> {
    let mut output = Vec::new();

    let mut files = Vec::from_iter(files.iter().map(|(path, body)| (*path, *body)));
    let mut files_with_offsets = Vec::new();

    files.sort_by_key(|(path, body)| (*body, *path));

    for (name, body) in files.iter() {
        let local_name = b"PK";
        let mut header = Vec::new();
        // 0x0000..0x0004: local file header signature
        header.extend_from_slice(b"PK\x03\x04");
        // 0x0004..0x0006: version needed to extract
        header.extend_from_slice(&2_0_u16.to_le_bytes());
        // 0x0006..0x0008: general purpose bit flag
        header.extend_from_slice(&[0x00; 2]);
        // 0x0008..0x000A: compression method
        header.extend_from_slice(&[0x00; 2]);
        // 0x000A..0x000C: modification time
        header.extend_from_slice(b"PK");
        // 0x000C..0x000E: modification date
        header.extend_from_slice(b"PK");
        // 0x000E..0x0012: checksum
        header.extend_from_slice(&zip_crc(body).to_le_bytes());
        // 0x0012..0x0016: compressed size
        header.extend_from_slice(&u32::try_from(body.len()).unwrap().to_le_bytes());
        // 0x0016..0x001A: uncompressed size
        header.extend_from_slice(&u32::try_from(body.len()).unwrap().to_le_bytes());
        // 0x001A..0x001E: file name length
        header.extend_from_slice(&u16::try_from(local_name.len()).unwrap().to_le_bytes());
        // 0x001E..0x0022: extra fields length
        header.extend_from_slice(&[0x00; 2]);
        // 0x0022: file name, followed by extra fields (we have none)
        header.extend_from_slice(local_name);
        let range = if !body.is_empty() {
            let header_range = write_aligned_pad_start(&mut output, &header, BLOCK_SIZE);
            let body_range = write_aligned_pad_end(&mut output, body, BLOCK_SIZE);
            header_range.start..body_range.end
        } else {
            write_aligned_pad_end(&mut output, &header, BLOCK_SIZE)
        };
        files_with_offsets.push((*name, *body, range.start));
    }

    let mut files = files_with_offsets;
    files.sort();

    let mut central_directory = Vec::new();
    for (name, body, header_offset) in files.iter() {
        let name = name.to_vec();
        let name_length = u16::try_from(name.len()).unwrap();
        let body_length = u32::try_from(body.len()).unwrap();
        let header_offset = u32::try_from(*header_offset).unwrap();
        let crc = zip_crc(body).to_le_bytes();
        let mut header = Vec::new();
        // 0x0000..0x0004: central file header signature
        header.extend_from_slice(b"PK\x01\x02");
        // 0x0004..0x0006: creator version and platform
        header.extend_from_slice(&20_u16.to_le_bytes());
        // 0x0006..0x0008: required version
        header.extend_from_slice(&20_u16.to_le_bytes());
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

    let archive_terminator = [0; 22];
    central_directory.extend_from_slice(&archive_terminator);

    let central_directory_range =
        write_aligned_pad_start(&mut output, &central_directory, BLOCK_SIZE);

    let final_len = output.len();
    let mut archive_terminator = &mut output[final_len - archive_terminator.len()..];

    let directory_offset = u32::try_from(central_directory_range.start).unwrap();
    let directory_count = u16::try_from(files.len()).unwrap();
    let directory_length =
        u32::try_from(central_directory_range.len() - archive_terminator.len()).unwrap();

    // 0x0000..0x0004: archive terminator signature
    archive_terminator.write_all(b"PK\x05\x06").unwrap();
    // 0x0004..0x0006: disk number
    archive_terminator.write_all(&[0x00; 2]).unwrap();
    // 0x0006..0x0008: disk number with central directory
    archive_terminator.write_all(&[0x00; 2]).unwrap();
    // 0x0008..0x000A: directory entries on disk
    archive_terminator
        .write_all(&directory_count.to_le_bytes())
        .unwrap();
    // 0x000A..0x000C: directory entries total
    archive_terminator
        .write_all(&directory_count.to_le_bytes())
        .unwrap();
    // 0x000C..0x0010: central directory byte length
    archive_terminator
        .write_all(&directory_length.to_le_bytes())
        .unwrap();
    // 0x0010..0x0014: central directory offset from start of archive
    archive_terminator
        .write_all(&directory_offset.to_le_bytes())
        .unwrap();
    // 0x0014..: archive comment length, followed by comment body (we have none)
    archive_terminator.write_all(&[0x00; 2]).unwrap();
    assert!(archive_terminator.is_empty());

    output
}

/// Writes `bytes` to `buffer`, padded with trailing zeroes to the next multiple
/// of `alignment`. Returns the range that `bytes` was written to in `buffer`,
/// excluding the padding.
fn write_aligned_pad_end(buffer: &mut Vec<u8>, bytes: &[u8], alignment: usize) -> Range<usize> {
    let index_before_data = buffer.len();

    buffer.extend_from_slice(bytes);

    let index_after_data = buffer.len();

    if index_after_data % alignment != 0 {
        let padding = alignment - (index_after_data % alignment);
        for _ in 0..padding {
            buffer.push(0);
        }
    }

    let _index_after_padding = buffer.len();

    index_before_data..index_after_data
}

/// Writes `bytes` to `buffer`, padded with leading zeroes to the next multiple
/// of `alignment`. Returns the range that `bytes` was written to in `buffer`,
/// excluding the padding.
fn write_aligned_pad_start(buffer: &mut Vec<u8>, bytes: &[u8], alignment: usize) -> Range<usize> {
    let index_before_padding = buffer.len();
    let unpadded_index_after_data = index_before_padding + bytes.len();
    if unpadded_index_after_data % alignment != 0 {
        let padding = alignment - (unpadded_index_after_data % alignment);
        for _ in 0..padding {
            buffer.push(0);
        }
    }
    let index_before_data = buffer.len();
    buffer.extend_from_slice(bytes);
    let index_after_data = buffer.len();
    index_before_data..index_after_data
}

fn zip_crc(bytes: &[u8]) -> u32 {
    const ZIP_CRC: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);
    let mut hasher = ZIP_CRC.digest();
    hasher.update(bytes);
    hasher.finalize()
}
