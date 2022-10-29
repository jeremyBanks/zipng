#![cfg_attr(debug_assertions, allow(unused))]
use std::collections::HashMap;
use std::convert::Infallible;
use std::env;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::format as f;
use std::hash::Hasher;
use std::str;

use arrayvec::ArrayVec;
use bstr::BStr;
use bstr::BString;
use derive_more::AsMut;
use derive_more::AsRef;
use derive_more::Deref;
use derive_more::From;
use derive_more::Into;
use digest::generic_array::GenericArray;
use digest::Digest;
use rusqlite_migration::Migrations;
use rusqlite_migration::M;
use serde::de;
use serde::de::SeqAccess;
use serde::de::Visitor;
use serde::ser;
use serde::ser::SerializeTuple;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use tracing::debug;
use tracing::error;
use tracing::info;
use tracing::instrument;
use tracing::metadata::LevelFilter;
use tracing::trace;
use tracing::warn;
use tracing_error::ErrorLayer;
use tracing_error::SpanTrace;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::EnvFilter;
use typenum::U20;

use crate::generic::default;

/// A blob ID is a value of 1 to 32 bytes representing a byte string
/// of arbitrary length.
///
/// It starts with an unsigned varint indicating the size.
///
/// If the size is small enough for the value itself to fit into the
/// remaining bytes, we do so. Otherwise, we the remaining bytes contain
/// as many bytes as possible from the beginning of the BLAKE3 digest of
/// the value.
///
/// For serialization, since the length is up-front we only need to
/// transmit as many bytes as neccessary for inline values. In particular,
/// note that an empty byte string is represented by a single zero byte.
/// If a fixed-length value is neccessary, the ID must be padded with trailing
/// zero bytes.
#[derive(Default, Clone, Copy, Eq, PartialOrd, PartialEq, Ord, Hash)]
pub struct BlobId {
    buffer: [u8; 32],
}

impl BlobId {
    pub fn new(slice: &[u8]) -> BlobId {
        let mut buffer = [0u8; 32];
        let mut remaining = &mut buffer[..];
        let split = leb128::write::unsigned(&mut remaining, slice.len() as u64).unwrap();

        if slice.len() < remaining.len() {
            remaining[..slice.len()].copy_from_slice(slice);
        } else {
            let digest = blake3::hash(slice);
            remaining.copy_from_slice(&digest.as_bytes()[..remaining.len()]);
        }
        BlobId { buffer }
    }

    pub fn len(&self) -> usize {
        let mut view = &self.buffer[..];
        leb128::read::unsigned(&mut view).unwrap() as usize
    }

    fn len_len(&self) -> usize {
        // ... this is always going to be 1 for inline values
        let mut view = &self.buffer[..];
        leb128::read::unsigned(&mut view).unwrap();
        self.buffer.len() - view.len()
    }

    pub fn to_bytes(&self) -> &[u8] {
        let len = self.len();
        if len < self.buffer.len() - self.len_len() {
            &self.buffer[..self.len_len() + len]
        } else {
            &self.buffer[..]
        }
    }

    pub fn from_reader(reader: impl std::io::Read) -> std::io::Result<BlobId> {
        todo!()
    }

    pub fn from_bytes(bytes: &[u8]) -> BlobId {
        let mut view = &bytes[..];
        let len = leb128::read::unsigned(&mut view).unwrap() as usize;
        let len_len = bytes.len() - view.len();
        let mut buffer = [0u8; 32];
        if len_len + len < 32 {
            buffer[..len_len + len].copy_from_slice(bytes);
        } else {
            buffer.copy_from_slice(bytes);
        }
        BlobId { buffer }

    }
}

impl Debug for BlobId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let len = self.len();
        if len == 0 {
            return write!(f, "0x00");
        }
        let len_len = self.len_len();
        let (before, after) = self.buffer.split_at(len_len);
        write!(
            f,
            "0x{}_{}",
            hex::encode_upper(before),
            hex::encode_upper(after)
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static SOME_BYTES: &[u8] = &[
        0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x01, 0x03, 0x07, 0x0F, 0x1F, 0x3F, 0x7F,
        0xFF, 0xFE, 0xFC, 0xF8, 0xF0, 0xE0, 0xC0, 0x80, 0x00, 0x01, 0x03, 0x07, 0x0F, 0x1F, 0x3F,
        0x7F, 0xFF, 0xFE, 0xFC, 0xF8, 0xF0, 0xE0, 0xC0, 0x80, 0x00, 0x01, 0x03, 0x07, 0x0F, 0x1F,
        0x3F, 0x7F, 0xFF, 0xFE, 0xFC, 0xF8, 0xF0, 0xE0, 0xC0, 0x80, 0x00, 0x01, 0x03, 0x07, 0x0F,
        0x1F, 0x3F, 0x7F, 0xFF, 0xFE, 0xFC, 0xF8, 0xF0, 0xE0, 0xC0, 0x80, 0x00, 0x01, 0x03, 0x07,
        0x0F, 0x1F, 0x3F, 0x7F, 0xFF, 0xFE, 0xFC, 0xF8, 0xF0, 0xE0, 0xC0, 0x80, 0x00, 0x01, 0x03,
        0x07, 0x0F, 0x1F, 0x3F, 0x7F, 0xFF, 0xFE, 0xFC, 0xF8, 0xF0, 0xE0, 0xC0, 0x80, 0x00, 0x01,
        0x03, 0x07, 0x0F, 0x1F, 0x3F, 0x7F, 0xFF, 0xFE, 0xFC, 0xF8, 0xF0, 0xE0, 0xC0, 0x80, 0x00,
    ];
    static SOME_WORDS: &str =
        "\
        alfa bravo charlie delta echo foxtrot golf hotel india juliet kilo lima mike november \
         oscar papa quebec romeo sierra tango uniform victor whiskey xray yankee zulu nada una \
         bisso terra karte panta soxi sette okto nove stop lorem ipsum dolor sit amet consectetur \
         adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua ut \
         enim ad minim veniam quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea \
         commodo consequat duis aute irure dolor in reprehenderit in voluptate velit esse cillum \
         dolore eu fugiat nulla pariatur excepteur sint occaecat cupidatat non proident sunt in \
         culpa qui officia deserunt mollit anim id est laborum ";
    fn samples() -> Vec<(&'static [u8], &'static str, &'static [u8], &'static str)> {
        vec![
            (&SOME_BYTES[..0], r#""""#, &[0], "[0,[]]"),
            (&SOME_BYTES[..1], r#""\x01""#, &[1, 1], "[1,[1]]"),
            (
                &SOME_BYTES[..16],
                r#""\x01\x02\x04\x08\x10 @\x80\x01\x03\x07\x0f\u{1f}?\x7f\xFF""#,
                &[
                    16, 1, 2, 4, 8, 16, 32, 64, 128, 1, 3, 7, 15, 31, 63, 127, 255,
                ],
                "[16,[1,2,4,8,16,32,64,128,1,3,7,15,31,63,127,255]]",
            ),
            (
                &SOME_BYTES[..27],
                r#""\x01\x02\x04\x08\x10 @\x80\x01\x03\x07\x0f\u{1f}?\x7f\xFF\xFE\xFC\xF8\xF0\xE0\xC0\x80\0\x01\x03\x07""#,
                &[
                    27, 1, 2, 4, 8, 16, 32, 64, 128, 1, 3, 7, 15, 31, 63, 127, 255, 254, 252, 248,
                    240, 224, 192, 128, 0, 1, 3, 7,
                ],
                "[27,[1,2,4,8,16,32,64,128,1,3,7,15,31,63,127,255,254,252,248,240,224,192,128,0,1,\
                 3,7]]",
            ),
            (
                &SOME_BYTES[..28],
                r#""\x01\x02\x04\x08\x10 @\x80\x01\x03\x07\x0f\u{1f}?\x7f\xFF\xFE\xFC\xF8\xF0\xE0\xC0\x80\0\x01\x03\x07\x0f""#,
                &[
                    28, 1, 2, 4, 8, 16, 32, 64, 128, 1, 3, 7, 15, 31, 63, 127, 255, 254, 252, 248,
                    240, 224, 192, 128, 0, 1, 3, 7, 15,
                ],
                "[28,[1,2,4,8,16,32,64,128,1,3,7,15,31,63,127,255,254,252,248,240,224,192,128,0,1,\
                 3,7,15]]",
            ),
            (
                &SOME_BYTES[..29],
                "0x1DF3712C209DADDA7326F423FE6D2C45F34C3814D24174DA3B16BE52F9",
                &[
                    29, 243, 113, 44, 32, 157, 173, 218, 115, 38, 244, 35, 254, 109, 44, 69, 243,
                    76, 56, 20, 210, 65, 116, 218, 59, 22, 190, 82, 249,
                ],
                "[29,[243,113,44,32,157,173,218,115,38,244,35,254,109,44,69,243,76,56,20,210,65,\
                 116,218,59,22,190,82,249]]",
            ),
            (
                &SOME_BYTES[..],
                "0x78BABA5B349C4B20B1582859B0BAB5D209C67FAFBF744D2CEA80EA8407",
                &[
                    120, 186, 186, 91, 52, 156, 75, 32, 177, 88, 40, 89, 176, 186, 181, 210, 9,
                    198, 127, 175, 191, 116, 77, 44, 234, 128, 234, 132, 7,
                ],
                "[120,[186,186,91,52,156,75,32,177,88,40,89,176,186,181,210,9,198,127,175,191,116,\
                 77,44,234,128,234,132,7]]",
            ),
            (SOME_WORDS[..0].as_bytes(), r#""""#, &[0], "[0,[]]"),
            (SOME_WORDS[..1].as_bytes(), r#""a""#, &[1, 97], "[1,[97]]"),
            (
                SOME_WORDS[..16].as_bytes(),
                r#""alfa bravo charl""#,
                &[
                    16, 97, 108, 102, 97, 32, 98, 114, 97, 118, 111, 32, 99, 104, 97, 114, 108,
                ],
                "[16,[97,108,102,97,32,98,114,97,118,111,32,99,104,97,114,108]]",
            ),
            (
                SOME_WORDS[..27].as_bytes(),
                r#""alfa bravo charlie delta ec""#,
                &[
                    27, 97, 108, 102, 97, 32, 98, 114, 97, 118, 111, 32, 99, 104, 97, 114, 108,
                    105, 101, 32, 100, 101, 108, 116, 97, 32, 101, 99,
                ],
                "[27,[97,108,102,97,32,98,114,97,118,111,32,99,104,97,114,108,105,101,32,100,101,\
                 108,116,97,32,101,99]]",
            ),
            (
                SOME_WORDS[..28].as_bytes(),
                r#""alfa bravo charlie delta ech""#,
                &[
                    28, 97, 108, 102, 97, 32, 98, 114, 97, 118, 111, 32, 99, 104, 97, 114, 108,
                    105, 101, 32, 100, 101, 108, 116, 97, 32, 101, 99, 104,
                ],
                "[28,[97,108,102,97,32,98,114,97,118,111,32,99,104,97,114,108,105,101,32,100,101,\
                 108,116,97,32,101,99,104]]",
            ),
            (
                SOME_WORDS[..29].as_bytes(),
                "0x1D670342D28AEC4885E85E9A623B2B0A2A1380B2E66568492EED95B170",
                &[
                    29, 103, 3, 66, 210, 138, 236, 72, 133, 232, 94, 154, 98, 59, 43, 10, 42, 19,
                    128, 178, 230, 101, 104, 73, 46, 237, 149, 177, 112,
                ],
                "[29,[103,3,66,210,138,236,72,133,232,94,154,98,59,43,10,42,19,128,178,230,101,\
                 104,73,46,237,149,177,112]]",
            ),
            (
                SOME_WORDS[..].as_bytes(),
                "0x9405C1C46B79DCBFF80865BF1DEBFE5306C0CD49C01392CAD7458136FBB2",
                &[
                    148, 5, 193, 196, 107, 121, 220, 191, 248, 8, 101, 191, 29, 235, 254, 83, 6,
                    192, 205, 73, 192, 19, 146, 202, 215, 69, 129, 54, 251, 178,
                ],
                "[660,[193,196,107,121,220,191,248,8,101,191,29,235,254,83,6,192,205,73,192,19,\
                 146,202,215,69,129,54,251,178]]",
            ),
        ]
    }

    #[test]
    fn test_blob_id_debug() {
        for (bytes, debug, _serialized_bytes, _serialized_text) in samples() {
            let blob_id = BlobId::new(bytes);
            assert_eq!(format!("{blob_id:?}"), debug);
        }
    }

    #[test]
    fn test_blob_id_serialize() {
        for (bytes, _debug, serialized_bytes, _serialized_text) in samples() {
            let blob_id = BlobId::new(bytes);
            let serialized = postcard::to_allocvec(&blob_id).unwrap();
            let deserialized = postcard::from_bytes(&serialized).unwrap();
            assert_eq!(blob_id, deserialized);
            assert_eq!(serialized_bytes, serialized);
        }
    }

    #[test]
    fn test_blob_id_serialize_as_text() {
        for (bytes, _debug, _serialized_bytes, serialized_text) in samples() {
            let blob_id = BlobId::new(bytes);
            let serialized = serde_json::to_string(&blob_id).unwrap();
            let deserialized: BlobId = serde_json::from_str(&serialized).unwrap();
            assert_eq!(blob_id, deserialized);
            assert_eq!(serialized_text, serialized);
        }
    }
}
