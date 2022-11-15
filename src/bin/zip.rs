#![allow(clippy::unusual_byte_groupings)]
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::io::Write;
use std::iter::repeat;
use std::ops::Range;

use crc::Crc;
use crc::CRC_32_ISO_HDLC;

pub fn main() {
    let pairs = [
        ("empty.txt", ""),
        ("nothing.txt", ""),
        ("nothing/nothing.txt", ""),
        ("README.txt", "hello, world!"),
    ];

    // collect into btree map then call zip, then write to target/test.zip in cwd
    let pairs: BTreeMap<Vec<u8>, Vec<u8>> = pairs
        .iter()
        .map(|(k, v)| (k.as_bytes().to_vec(), v.as_bytes().to_vec()))
        .collect();
    let file = zip(pairs).unwrap();
    std::fs::write("target/test.zip", file).unwrap();
}

pub const ALIGNMENT: usize = 128; // 1024;

const ZIP_VERSION: [u8; 2] = 20_u16.to_le_bytes();
const LOCAL_FILE_SIGNATURE: [u8; 4] = *b"PK\x03\x04";
const CENTRAL_FILE_SIGNATURE: [u8; 4] = *b"PK\x01\x02";
const ARCHIVE_TERMINATOR_SIGNATURE: [u8; 4] = *b"PK\x05\x06";
const LOCAL_FILE_HEADER_SIZE: usize = 30;

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
        rest.write_all(&crc(b""))?;
        rest.write_all(&[0_u8; 12])?;
        assert!(rest.is_empty(), "{:?} bytes missing", rest.len());
        header
    });

    for contents in unindexed_blobs {
        write_aligned_pad_start(&mut output, &{
            let length = u32::try_from(contents.len()).unwrap().to_le_bytes();
            let mut header = [00_u8; LOCAL_FILE_HEADER_SIZE];
            let mut rest = &mut header[..];
            rest.write_all(&LOCAL_FILE_SIGNATURE)?;
            rest.write_all(&ZIP_VERSION)?;
            rest.write_all(&[0_u8; 8])?;
            rest.write_all(&crc(&contents))?;
            rest.write_all(&length)?;
            rest.write_all(&length)?;
            rest.write_all(&[0_u8; 4])?;
            assert!(rest.is_empty(), "{:?} bytes missing", rest.len());
            header
        });
        blob_indices.insert(contents, write_aligned_pad_end(&mut output, contents));
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
            rest.write_all(&crc(&contents))?;
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

        let mut terminator = Vec::new();
        let mut rest = &mut terminator;

        let offset = u32::try_from(directory_start).unwrap().to_le_bytes();
        let count = u32::try_from(data.len()).unwrap().to_le_bytes();
        let length = u32::try_from(central_directory.len())
            .unwrap()
            .to_le_bytes();

        rest.write_all(&ARCHIVE_TERMINATOR_SIGNATURE)?;
        rest.write_all(&[0_u8; 4])?;
        rest.write_all(&count)?;
        rest.write_all(&count)?;
        rest.write_all(&length)?;
        rest.write_all(&offset)?;
        rest.write_all(&[0_u8; 4])?;

        central_directory
            .into_iter()
            .chain(terminator.into_iter())
            .collect::<Vec<u8>>()
    });

    Ok(output)
}

fn crc(bytes: &[u8]) -> [u8; 4] {
    let zip_crc = Crc::<u32>::new(&CRC_32_ISO_HDLC);
    let mut hasher = zip_crc.digest();
    hasher.update(bytes);
    hasher.finalize().to_le_bytes()
}

fn write_aligned_pad_end(buffer: &mut Vec<u8>, bytes: &[u8]) -> Range<usize> {
    let start = buffer.len();
    buffer.extend(bytes);
    let end = buffer.len();
    write_aligned_pad_start(buffer, b"");
    start..end
}

fn write_aligned_pad_start(buffer: &mut Vec<u8>, bytes: &[u8]) -> Range<usize> {
    let before = buffer.len();
    let plus_len = before + bytes.len();
    if plus_len % ALIGNMENT != 0 {
        let padding = ALIGNMENT - (plus_len % ALIGNMENT);
        buffer.extend(repeat(0).take(padding));
    }
    let start = buffer.len();
    buffer.extend(bytes);
    let end = buffer.len();
    start..end
}
