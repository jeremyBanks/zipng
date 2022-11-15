#![allow(clippy::unusual_byte_groupings)]
use std::collections::BTreeMap;
use std::fmt::Display;
use std::io::Write;
use std::iter::repeat;

use crc::Algorithm;
use crc::Crc;
use crc::CRC_32_ISO_HDLC;

pub fn crc(bytes: &[u8]) -> [u8; 4] {
    let mut crc = Crc::<u32>::new(&CRC_32_ISO_HDLC).digest();
    crc.update(bytes);
    crc.finalize().to_le_bytes()
}

const ALIGNMENT: usize = 1024;
const ZIP_VERSION: [u8; 2] = 20_u16.to_le_bytes();
const GLOBAL_DIRECTORY_SIGNATURE: [u8; 4] = *b"PK\x01\x02";
const LOCAL_FILE_SIGNATURE: [u8; 4] = *b"PK\x03\x04";
const LOCAL_FILE_FLAGS: [u8; 2] = 0_u16.to_le_bytes();
const LOCAL_FILE_COMPRESSION: [u8; 2] = 0_u16.to_le_bytes();
const LOCAL_FILE_MODIFICATION_TIME: [u8; 2] = 0b_01010_010_0100_1010_u16.to_le_bytes(); // 20:20:20
const LOCAL_FILE_MODIFICATION_DATE: [u8; 2] = 0b_10100_0001_101000_u16.to_le_bytes(); // 2020-02-20
const LOCAL_FILE_NAME_LENGTH: [u8; 2] = 64_u16.to_le_bytes();
const LOCAL_EXTRA_FIELD_LENGTH: [u8; 2] = 0_u16.to_le_bytes();

fn write_aligned_right_pad(buffer: &mut Vec<u8>, bytes: &[u8]) {
    buffer.extend(bytes);
    write_aligned_left_pad(buffer, b"")
}

fn write_aligned_left_pad(buffer: &mut Vec<u8>, bytes: &[u8]) {
    let offset = buffer.len();
    let offset_after = offset + bytes.len();
    if offset_after % ALIGNMENT != 0 {
        let padding = ALIGNMENT - (offset_after % ALIGNMENT);
        buffer.extend(repeat(0).take(padding));
    }
    buffer.extend(bytes);
}

pub fn write_zip(data: BTreeMap<Vec<Vec<u8>>, Vec<u8>>) -> Result<Vec<u8>, eyre::Report> {
    let mut output = Vec::new();

    let contents = BTreeMap::from_iter(
        data.values()
            .map(|content| (blake3::hash(content).to_hex().to_ascii_uppercase(), content)),
    );

    //
    let mut indicies = ();

    write_aligned_right_pad(&mut output, &{
        // Unused empty nameless file, at the beginning of the zip file.
        // This is only for the benefit of any applications that sniff for the local
        // file signature at the beginning of the file.
        let mut header = Vec::new();
        let length = u32::try_from(contents.len()).unwrap().to_le_bytes();
        output.write_all(&LOCAL_FILE_SIGNATURE)?;
        output.write_all(&LOCAL_FILE_FLAGS)?;
        output.write_all(&LOCAL_FILE_COMPRESSION)?;
        output.write_all(&LOCAL_FILE_MODIFICATION_TIME)?;
        output.write_all(&LOCAL_FILE_MODIFICATION_DATE)?;
        output.write_all(&crc(b""))?;
        output.write_all(&0_u16.to_le_bytes())?;
        output.write_all(&0_u16.to_le_bytes())?;
        output.write_all(&0_u16.to_le_bytes())?;
        output.write_all(&LOCAL_EXTRA_FIELD_LENGTH)?;
        header
    });

    for (digest, contents) in contents {
        write_aligned_left_pad(&mut output, &{
            let mut header = Vec::new();
            let length = u32::try_from(contents.len()).unwrap().to_le_bytes();
            header.write_all(&LOCAL_FILE_SIGNATURE)?;
            header.write_all(&LOCAL_FILE_FLAGS)?;
            header.write_all(&LOCAL_FILE_COMPRESSION)?;
            header.write_all(&LOCAL_FILE_MODIFICATION_TIME)?;
            header.write_all(&LOCAL_FILE_MODIFICATION_DATE)?;
            header.write_all(&crc(contents))?;
            header.write_all(&length)?;
            header.write_all(&length)?;
            header.write_all(&LOCAL_FILE_NAME_LENGTH)?;
            header.write_all(&LOCAL_EXTRA_FIELD_LENGTH)?;
            header.write_all(digest.as_bytes())?;
            header
        });
        write_aligned_right_pad(&mut output, contents);
    }

    // central directory header
    output.write_all(b"PK\x01\x02")?;

    Ok(output)
}

// TODO: manual simple zip encoder
//
// each zip file's local name is the uppercase hex blake3 of the file's
// contents, and the actual filenames are only stored in the central directory
// (where they're implicitly deduplicated)
//
// each file immediately starts with a zero-length zip file

type KeyPath = Vec<Vec<u8>>;
const KEY_PATH_SEPARATOR: u8 = b'/';
const ESCAPE_PREFIX: &[u8] = b".B0x";
const ESCAPE_ALPHABET: &[u8] = b"0123456789AbCdEf";
const MUST_ESCAPE: &[u8] = &[0x00, b'/', b'\\', b':', b'>', b'<', b'\'', b'"'];
const CHUNK: usize = 1024;

fn must_escape(piece: &[u8]) -> bool {
    if piece.is_empty() {
        return true;
    }

    if let Some(rest) = piece.strip_prefix(ESCAPE_PREFIX) {
        if rest.len() % 2 == 0 && rest.iter().all(|&b| b"0123456789ABCDEF".contains(&b)) {
            warn!("Are we double-escaping? {piece:?}");
            return true;
        }
    }

    piece.iter().any(|&b| MUST_ESCAPE.contains(&b))
}

fn key_path_to_path(key_path: &KeyPath) -> PathBuf {}

fn path_to_key_path(path: &Path) -> KeyPath {}

/// Creates a zip file, with files stored uncompressed, with data starting at an
/// offset of 1024, and file data zero-padded out to a multiple of 1024.
pub fn zip(contents: BTreeMap<Vec<Vec<u8>>, Vec<u8>>) -> Vec<u8> {
    let mut buffer = Vec::new();
    let mut writer = zip::ZipWriter::new(&mut buffer);

    for (path, data) in contents {
        let offset = writer.start_file_aligned(KIBI);
        let mut options = zip::write::FileOptions::default();
        options.unix_permissions(0o644);
        writer.start_file_from_path(path, options).unwrap();
        writer.write_all(&data).unwrap();
    }
}

/*
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::Path;
use std::path::PathBuf;

use tracing::warn;

use crate::panic;

pub const ALIGNMENT: usize = 1024;
pub const PK_ZIP_VERSION: u8 = 20;
pub const FILE_HEADER: [u8; 4] = b"PK\x03\x04";

pub fn write_zip(data: BTreeMap<Vec<Vec<u8>>, Vec<u8>>) -> Result<Vec<u8>, panic> {
    let mut offset: usize = 0;
    let start_offset = offset;
    let mut buffer = Vec::new();

    let contents = BTreeMap::from_iter(
        data.iter()
            .map(|content| (blake3::hash(content).to_hex().to_ascii_uppercase(), content)),
    );

    for (digest, contents) in contents {
        buffer.write_all(b"PK\x03\x04")?;
        buffer.write_all(&[PK_ZIP_VERSION])?;
    }

    // central directory header
    buffer.write_all(b"PK\x01\x02")?;

    Ok(buffer)
}

// TODO: manual simple zip encoder
//
// each zip file's local name is the uppercase hex blake3 of the file's
// contents, and the actual filenames are only stored in the central directory
// (where they're implicitly deduplicated)
//
// each file immediately starts with a zero-length zip file

type KeyPath = Vec<Vec<u8>>;
const KEY_PATH_SEPARATOR: u8 = b'/';
const ESCAPE_PREFIX: &[u8] = b".B0x";
const ESCAPE_ALPHABET: &[u8] = b"0123456789AbCdEf";
const MUST_ESCAPE: &[u8] = &[0x00, b'/', b'\\', b':', b'>', b'<', b'\'', b'"'];
const CHUNK: usize = 1024;

fn must_escape(piece: &[u8]) -> bool {
    if piece.is_empty() {
        return true;
    }

    if let Some(rest) = piece.strip_prefix(ESCAPE_PREFIX) {
        if rest.len() % 2 == 0 && rest.iter().all(|&b| b"0123456789ABCDEF".contains(&b)) {
            warn!("Are we double-escaping? {piece:?}");
            return true;
        }
    }

    piece.iter().any(|&b| MUST_ESCAPE.contains(&b))
}

fn key_path_to_path(key_path: &KeyPath) -> PathBuf {}

fn path_to_key_path(path: &Path) -> KeyPath {}

/// Creates a zip file, with files stored uncompressed, with data starting at an
/// offset of 1024, and file data zero-padded out to a multiple of 1024.
pub fn zip(contents: BTreeMap<Vec<Vec<u8>>, Vec<u8>>) -> Vec<u8> {
    let mut buffer = Vec::new();
    let mut writer = zip::ZipWriter::new(&mut buffer);

    for (path, data) in contents {
        let offset = writer.start_file_aligned(KIBI);
        let mut options = zip::write::FileOptions::default();
        options.unix_permissions(0o644);
        writer.start_file_from_path(path, options).unwrap();
        writer.write_all(&data).unwrap();
    }
}
*/
