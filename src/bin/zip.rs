#![allow(clippy::unusual_byte_groupings)]
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::io::Write;
use std::iter::repeat;
use std::ops::Range;

use crc::Crc;
use crc::CRC_32_ISO_HDLC;

const BLOCK_SIZE: usize = 1024;

pub fn main() {
    let pairs: &[(&[u8], &[u8])] = &[
        (b"empty.txt", b""),
        (b"1.txt", b"1"),
        (b"2.txt", b"22"),
        (b"333.txt", b"333"),
        (b"nothing.txt", b""),
        (b"nothing/nothing.txt", b""),
        (b"README.txt", b"hello, world!"),
    ];

    let file = zip(&pairs);
    std::fs::write("target/test.zip", file).unwrap();
}

const ZIP_VERSION: [u8; 2] = 20_u16.to_le_bytes();
const LOCAL_FILE_SIGNATURE: [u8; 4] = *b"PK\x03\x04";
const ARCHIVE_TERMINATOR_SIGNATURE: [u8; 4] = *b"PK\x05\x06";
const LOCAL_FILE_HEADER_SIZE: usize = 30;

fn zip(files: &[(&[u8], &[u8])]) -> Vec<u8> {
    let mut output = Vec::new();

    let mut bodies = BTreeSet::from_iter(files.iter().map(|(_name, body)| *body));
    bodies.insert(&[]);

    let mut body_offsets = BTreeMap::new();

    for (i, body) in bodies.iter().enumerate() {
        let mut name = Vec::new();
        static DIGITS: &[u8; 36] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let mut remainder = i;
        while remainder > 0 {
            let digit = remainder % DIGITS.len();
            remainder /= DIGITS.len();
            name.push(DIGITS[digit]);
        }
        while name.len() < 2 {
            name.insert(0, b'0');
        }
        body_offsets.insert(body, write_blob(&mut output, &name, body));
    }

    let mut central_directory = Vec::new();
    for (name, body) in files {
        let header_offset = u32::try_from(body_offsets[body].start).unwrap();
        let name = name.to_vec();
        let name_length = u16::try_from(name.len()).unwrap();
        let body_length = u32::try_from(body.len()).unwrap();
        let crc = zip_crc(body).to_le_bytes();
        let mut header = Vec::new();
        // 0x0000..0x0004: central file header signature
        header.extend_from_slice(b"PK\x01\x02");
        // 0x0004..0x0006: creator version and platform
        header.extend_from_slice(&ZIP_VERSION);
        // 0x0006..0x0008: required version
        header.extend_from_slice(&ZIP_VERSION);
        // 0x0008..0x000A: general purpose bit flag
        header.extend_from_slice(&[0x00; 2]);
        // 0x000A..0x000C: compression method
        header.extend_from_slice(&[0x00; 2]);
        // 0x000C..0x000E: modification time
        header.extend_from_slice(&[0x00; 2]);
        // 0x000E..0x0010: modification date
        header.extend_from_slice(&[0x00; 2]);
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

    let central_directory_range = write_aligned_pad_end(&mut output, &central_directory, BLOCK_SIZE);

    let directory_offset = u32::try_from(central_directory_range.start).unwrap();
    let directory_count = u16::try_from(files.len()).unwrap();
    let directory_length = u32::try_from(central_directory_range.len()).unwrap();

    let mut archive_terminator = Vec::new();
    // 0x0000..0x0004: archive terminator signature
    archive_terminator.extend_from_slice(b"PK\x05\x06");
    // 0x0004..0x0008: disk number
    archive_terminator.extend_from_slice(&[0x00; 2]);
    // 0x0008..0x000A: disk number with central directory
    archive_terminator.extend_from_slice(&[0x00; 2]);
    // 0x000A..0x000C: directory entries on disk
    archive_terminator.extend_from_slice(&directory_count.to_le_bytes());
    // 0x000C..0x000E: directory entries total
    archive_terminator.extend_from_slice(&directory_count.to_le_bytes());
    // 0x000E..0x0012: central directory byte length
    archive_terminator.extend_from_slice(&directory_length.to_le_bytes());
    // 0x0012..0x0016: central directory offset from start of archive
    archive_terminator.extend_from_slice(&directory_offset.to_le_bytes());
    // 0x0016..: archive comment length, followed by comment body (we have none)
    archive_terminator.extend_from_slice(&[0x00; 2]);


    write_aligned_pad_start(&mut output, &archive_terminator, BLOCK_SIZE);

    output
}

fn write_blob(buffer: &mut Vec<u8>, name: &[u8], body: &[u8]) -> Range<usize> {
    let mut header = Vec::new();
    write_blob_header(&mut header, name, body);
    if !body.is_empty() {
        let header_range = write_aligned_pad_start(buffer, &header, BLOCK_SIZE);
        let body_range = write_aligned_pad_end(buffer, body, BLOCK_SIZE);
        header_range.start..body_range.end
    } else {
        write_aligned_pad_end(buffer, &header, BLOCK_SIZE)
    }
}

fn write_blob_header(buffer: &mut Vec<u8>, name: &[u8], data: &[u8]) -> Range<usize> {
    let index_before = buffer.len();
    // 0x0000..0x0004: local file header signature
    buffer.extend_from_slice(b"PK\x03\x04");
    // 0x0004..0x0006: version needed to extract
    buffer.extend_from_slice(&2_0_u16.to_le_bytes());
    // 0x0006..0x0008: general purpose bit flag
    buffer.extend_from_slice(&[0x00, 0x00]);
    // 0x0008..0x000A: compression method
    buffer.extend_from_slice(&[0x00, 0x00]);
    // 0x000A..0x000C: modification time
    buffer.extend_from_slice(&[0x00, 0x00]);
    // 0x000C..0x000E: modification date
    buffer.extend_from_slice(&[0x00, 0x00]);
    // 0x000E..0x0012: checksum
    buffer.extend_from_slice(&zip_crc(data).to_le_bytes());
    // 0x0012..0x0016: compressed size
    buffer.extend_from_slice(&u32::try_from(data.len()).unwrap().to_le_bytes());
    // 0x0016..0x001A: uncompressed size
    buffer.extend_from_slice(&u32::try_from(data.len()).unwrap().to_le_bytes());
    // 0x001A..0x001E: file name length
    buffer.extend_from_slice(&u16::try_from(name.len()).unwrap().to_le_bytes());
    // 0x001E..0x0022: extra fields length
    buffer.extend_from_slice(&[0x00, 0x00]);
    // 0x0022: file name, followed by extra fields (we have none)
    buffer.extend_from_slice(name);
    let index_after = buffer.len();
    index_before..index_after
}

/// Writes `bytes` to `buffer`, padded with trailing zeroes to the next multiple of `alignment`.
/// Returns the range that `bytes` was written to in `buffer`, excluding the padding.
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

/// Writes `bytes` to `buffer`, padded with leading zeroes to the next multiple of `alignment`.
/// Returns the range that `bytes` was written to in `buffer`, excluding the padding.
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
    const ZIP_CRC: Crc::<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);
    let mut hasher = ZIP_CRC.digest();
    hasher.update(bytes);
    hasher.finalize()
}

/*

pub fn zip(data: BTreeMap<Vec<u8>, Vec<u8>>) -> Result<Vec<u8>, eyre::Report> {
    let mut output = Vec::new();

    let mut unindexed_blobs = BTreeSet::<&[u8]>::new();
    unindexed_blobs.extend(data.values().map(|v| v.as_slice()));
    let mut blob_indices = BTreeMap::<&[u8], Range<usize>>::new();

    unindexed_blobs.remove("".as_bytes());
    blob_indices.insert(b"", LOCAL_FILE_HEADER_SIZE..LOCAL_FILE_HEADER_SIZE);
    write_aligned_pad_end(&mut output, &{
        // We always have an empty file at the beginning of the zip file, for the sake
        // of any programs that are sniffing for a local file signature.
        let mut header = [00_u8; LOCAL_FILE_HEADER_SIZE];
        let mut rest = &mut header[..];
        rest.write_all(&LOCAL_FILE_SIGNATURE)?;
        rest.write_all(&ZIP_VERSION)?;
        rest.write_all(&[0_u8; 8])?;
        rest.write_all(&crc(b"").to_le_bytes())?;
        rest.write_all(&[0_u8; 12])?;
        assert!(rest.is_empty(), "{:?} bytes missing", rest.len());
        header
    }, KIBI);

    for contents in unindexed_blobs {
        write_aligned_pad_start(&mut output, &{
            let length = u32::try_from(contents.len()).unwrap().to_le_bytes();
            let mut header = [00_u8; LOCAL_FILE_HEADER_SIZE];
            let mut rest = &mut header[..];
            rest.write_all(&LOCAL_FILE_SIGNATURE)?;
            rest.write_all(&ZIP_VERSION)?;
            rest.write_all(&[0_u8; 8])?;
            rest.write_all(&crc(&contents).to_le_bytes())?;
            rest.write_all(&length)?;
            rest.write_all(&length)?;
            rest.write_all(&[0_u8; 4])?;
            assert!(rest.is_empty(), "{:?} bytes missing", rest.len());
            header
        }, KIBI);
        blob_indices.insert(contents, write_aligned_pad_end(&mut output, contents, KIBI));
    }

    let directory_start = output.len();

    write_aligned_pad_start(&mut output, &{
        let mut central_directory = Vec::new();
        let mut rest = &mut central_directory;
        for (name, contents) in data.iter() {
            let length = u32::try_from(contents.len()).unwrap().to_le_bytes();
            rest.write_all(&CENTRAL_FILE_SIGNATURE)?;
            rest.write_all(&ZIP_VERSION)?;
            rest.write_all(&ZIP_VERSION)?;
            rest.write_all(&[0_u8; 8])?;
            rest.write_all(&crc(&contents).to_le_bytes())?;
            rest.write_all(&length)?;
            rest.write_all(&length)?;
            rest.write_all(&u16::try_from(name.len()).unwrap().to_le_bytes())?;
            rest.write_all(&[0_u8; 6])?;
            rest.write_all(
                &u32::try_from(blob_indices[contents.as_slice()].start - LOCAL_FILE_HEADER_SIZE)
                    .unwrap()
                    .to_le_bytes(),
            )?;
            rest.write_all(name)?;
        }

        let mut terminator = [00_u8; 22];

        let offset = u32::try_from(directory_start).unwrap().to_le_bytes();
        let count = u16::try_from(data.len()).unwrap().to_le_bytes();
        let length = u32::try_from(central_directory.len() + terminator.len())
            .unwrap()
            .to_le_bytes();

        let mut rest = &mut terminator[..];

        rest.write_all(&ARCHIVE_TERMINATOR_SIGNATURE)?;
        rest.write_all(&[0_u8; 4])?;
        rest.write_all(&count)?;
        rest.write_all(&count)?;
        rest.write_all(&length)?;
        rest.write_all(&offset)?;
        rest.write_all(&[0_u8; 2])?;
        assert!(rest.is_empty(), "{:?} bytes missing", rest.len());

        central_directory
            .into_iter()
            .chain(terminator.into_iter())
            .collect::<Vec<u8>>()
    }, KIBI);

    Ok(output)
} */
