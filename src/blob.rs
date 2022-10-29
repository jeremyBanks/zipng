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
/// It starts with an unsigned varint indicating the byte length.
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
    buffer: [u8; BlobId::BUFFER],
}

impl BlobId {
    pub const BUFFER: usize = 32;

    pub fn new(slice: &[u8]) -> BlobId {
        let mut buffer = [0u8; BlobId::BUFFER];
        let mut remaining = &mut buffer[..];
        let split = leb128::write::unsigned(&mut remaining, slice.len() as u64).unwrap();

        if slice.len() <= remaining.len() {
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
        let mut view = &self.buffer[..];
        leb128::read::unsigned(&mut view).unwrap();
        self.buffer.len() - view.len()
    }

    pub fn to_bytes(&self) -> &[u8] {
        let len = self.len();
        if len < self.buffer.len() - self.len_len() {
            &self.buffer[..(self.len_len() + len).max(BlobId::BUFFER)]
        } else {
            &self.buffer[..]
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> BlobId {
        let mut view = &bytes[..];
        let len = leb128::read::unsigned(&mut view).unwrap() as usize;
        let len_len = bytes.len() - view.len();
        let mut buffer = [0u8; BlobId::BUFFER];
        if len_len + len < BlobId::BUFFER {
            buffer[..len_len + len].copy_from_slice(bytes);
        } else {
            buffer.copy_from_slice(bytes);
        }
        BlobId { buffer }
    }
}

impl Serialize for BlobId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let buffer_len = 1 + BlobId::BUFFER - self.len_len();
        let mut tuple = serializer.serialize_tuple(buffer_len)?;
        tuple.serialize_element(&self.len())?;
        for byte in &self.buffer[self.len_len()..(self.len_len() + self.len()).min(BlobId::BUFFER)]
        {
            tuple.serialize_element(byte)?;
        }
        tuple.end()
    }
}

impl<'de> Deserialize<'de> for BlobId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BlobIdVisitor;

        impl<'de> Visitor<'de> for BlobIdVisitor {
            type Value = BlobId;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a byte array of length 1 to 32")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let length: u64 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &"missing length prefix"))?;

                let mut buffer = [0u8; BlobId::BUFFER];
                let mut remaining = &mut buffer[..];
                let split = leb128::write::unsigned(&mut remaining, length).unwrap();

                for (i, b) in remaining.iter_mut().enumerate().take(length as usize) {
                    *b = seq
                        .next_element()?
                        .ok_or_else(|| de::Error::invalid_length(split + i, &"body too short"))?;
                }

                Ok(BlobId { buffer })
            }
        }

        deserializer.deserialize_tuple(BlobId::BUFFER, BlobIdVisitor)
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
            hex::encode_upper(&after[..len.min(after.len())])
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
            (&SOME_BYTES[..0], "0x00", &[0], "[0]"),
            (&SOME_BYTES[..1], "0x01_01", &[1, 1], "[1,1]"),
            (&SOME_BYTES[..2], "0x02_0102", &[2, 1, 2], "[2,1,2]"),
            (
                &SOME_BYTES[..16],
                "0x10_01020408102040800103070F1F3F7FFF",
                &[
                    16, 1, 2, 4, 8, 16, 32, 64, 128, 1, 3, 7, 15, 31, 63, 127, 255,
                ],
                "[16,1,2,4,8,16,32,64,128,1,3,7,15,31,63,127,255]",
            ),
            (
                &SOME_BYTES[..27],
                "0x1B_01020408102040800103070F1F3F7FFFFEFCF8F0E0C08000010307",
                &[
                    27, 1, 2, 4, 8, 16, 32, 64, 128, 1, 3, 7, 15, 31, 63, 127, 255, 254, 252, 248,
                    240, 224, 192, 128, 0, 1, 3, 7,
                ],
                "[27,1,2,4,8,16,32,64,128,1,3,7,15,31,63,127,255,254,252,248,240,224,192,128,0,1,\
                 3,7]",
            ),
            (
                &SOME_BYTES[..28],
                "0x1C_01020408102040800103070F1F3F7FFFFEFCF8F0E0C080000103070F",
                &[
                    28, 1, 2, 4, 8, 16, 32, 64, 128, 1, 3, 7, 15, 31, 63, 127, 255, 254, 252, 248,
                    240, 224, 192, 128, 0, 1, 3, 7, 15,
                ],
                "[28,1,2,4,8,16,32,64,128,1,3,7,15,31,63,127,255,254,252,248,240,224,192,128,0,1,\
                 3,7,15]",
            ),
            (
                &SOME_BYTES[..29],
                "0x1D_01020408102040800103070F1F3F7FFFFEFCF8F0E0C080000103070F1F",
                &[
                    29, 1, 2, 4, 8, 16, 32, 64, 128, 1, 3, 7, 15, 31, 63, 127, 255, 254, 252, 248,
                    240, 224, 192, 128, 0, 1, 3, 7, 15, 31,
                ],
                "[29,1,2,4,8,16,32,64,128,1,3,7,15,31,63,127,255,254,252,248,240,224,192,128,0,1,\
                 3,7,15,31]",
            ),
            (
                &SOME_BYTES[..30],
                "0x1E_01020408102040800103070F1F3F7FFFFEFCF8F0E0C080000103070F1F3F",
                &[
                    30, 1, 2, 4, 8, 16, 32, 64, 128, 1, 3, 7, 15, 31, 63, 127, 255, 254, 252, 248,
                    240, 224, 192, 128, 0, 1, 3, 7, 15, 31, 63,
                ],
                "[30,1,2,4,8,16,32,64,128,1,3,7,15,31,63,127,255,254,252,248,240,224,192,128,0,1,\
                 3,7,15,31,63]",
            ),
            (
                &SOME_BYTES[..31],
                "0x1F_01020408102040800103070F1F3F7FFFFEFCF8F0E0C080000103070F1F3F7F",
                &[
                    31, 1, 2, 4, 8, 16, 32, 64, 128, 1, 3, 7, 15, 31, 63, 127, 255, 254, 252, 248,
                    240, 224, 192, 128, 0, 1, 3, 7, 15, 31, 63, 127,
                ],
                "[31,1,2,4,8,16,32,64,128,1,3,7,15,31,63,127,255,254,252,248,240,224,192,128,0,1,\
                 3,7,15,31,63,127]",
            ),
            (
                &SOME_BYTES[..32],
                "0x20_49659399D7B5F6677FA21F90557C80448E89BFBC169147E0571C3C7A674907",
                &[
                    32, 73, 101, 147, 153, 215, 181, 246, 103, 127, 162, 31, 144, 85, 124, 128, 68,
                    142, 137, 191, 188, 22, 145, 71, 224, 87, 28, 60, 122, 103, 73, 7,
                ],
                "[32,73,101,147,153,215,181,246,103,127,162,31,144,85,124,128,68,142,137,191,188,\
                 22,145,71,224,87,28,60,122,103,73,7]",
            ),
            (
                &SOME_BYTES[..33],
                "0x21_D5549F25DD3D1837A51C437C61156B8B34CFD49EDF1933F5006E27FBB572E6",
                &[
                    33, 213, 84, 159, 37, 221, 61, 24, 55, 165, 28, 67, 124, 97, 21, 107, 139, 52,
                    207, 212, 158, 223, 25, 51, 245, 0, 110, 39, 251, 181, 114, 230,
                ],
                "[33,213,84,159,37,221,61,24,55,165,28,67,124,97,21,107,139,52,207,212,158,223,25,\
                 51,245,0,110,39,251,181,114,230]",
            ),
            (
                &SOME_BYTES[..],
                "0x78_BABA5B349C4B20B1582859B0BAB5D209C67FAFBF744D2CEA80EA84076A6BC3",
                &[
                    120, 186, 186, 91, 52, 156, 75, 32, 177, 88, 40, 89, 176, 186, 181, 210, 9,
                    198, 127, 175, 191, 116, 77, 44, 234, 128, 234, 132, 7, 106, 107, 195,
                ],
                "[120,186,186,91,52,156,75,32,177,88,40,89,176,186,181,210,9,198,127,175,191,116,\
                 77,44,234,128,234,132,7,106,107,195]",
            ),
            (SOME_WORDS[..0].as_bytes(), "0x00", &[0], "[0]"),
            (SOME_WORDS[..1].as_bytes(), "0x01_61", &[1, 97], "[1,97]"),
            (
                SOME_WORDS[..16].as_bytes(),
                "0x10_616C666120627261766F20636861726C",
                &[
                    16, 97, 108, 102, 97, 32, 98, 114, 97, 118, 111, 32, 99, 104, 97, 114, 108,
                ],
                "[16,97,108,102,97,32,98,114,97,118,111,32,99,104,97,114,108]",
            ),
            (
                SOME_WORDS[..27].as_bytes(),
                "0x1B_616C666120627261766F20636861726C69652064656C7461206563",
                &[
                    27, 97, 108, 102, 97, 32, 98, 114, 97, 118, 111, 32, 99, 104, 97, 114, 108,
                    105, 101, 32, 100, 101, 108, 116, 97, 32, 101, 99,
                ],
                "[27,97,108,102,97,32,98,114,97,118,111,32,99,104,97,114,108,105,101,32,100,101,\
                 108,116,97,32,101,99]",
            ),
            (
                SOME_WORDS[..28].as_bytes(),
                "0x1C_616C666120627261766F20636861726C69652064656C746120656368",
                &[
                    28, 97, 108, 102, 97, 32, 98, 114, 97, 118, 111, 32, 99, 104, 97, 114, 108,
                    105, 101, 32, 100, 101, 108, 116, 97, 32, 101, 99, 104,
                ],
                "[28,97,108,102,97,32,98,114,97,118,111,32,99,104,97,114,108,105,101,32,100,101,\
                 108,116,97,32,101,99,104]",
            ),
            (
                SOME_WORDS[..29].as_bytes(),
                "0x1D_616C666120627261766F20636861726C69652064656C7461206563686F",
                &[
                    29, 97, 108, 102, 97, 32, 98, 114, 97, 118, 111, 32, 99, 104, 97, 114, 108,
                    105, 101, 32, 100, 101, 108, 116, 97, 32, 101, 99, 104, 111,
                ],
                "[29,97,108,102,97,32,98,114,97,118,111,32,99,104,97,114,108,105,101,32,100,101,\
                 108,116,97,32,101,99,104,111]",
            ),
            (
                SOME_WORDS[..30].as_bytes(),
                "0x1E_616C666120627261766F20636861726C69652064656C7461206563686F20",
                &[
                    30, 97, 108, 102, 97, 32, 98, 114, 97, 118, 111, 32, 99, 104, 97, 114, 108,
                    105, 101, 32, 100, 101, 108, 116, 97, 32, 101, 99, 104, 111, 32,
                ],
                "[30,97,108,102,97,32,98,114,97,118,111,32,99,104,97,114,108,105,101,32,100,101,\
                 108,116,97,32,101,99,104,111,32]",
            ),
            (
                SOME_WORDS[..31].as_bytes(),
                "0x1F_616C666120627261766F20636861726C69652064656C7461206563686F2066",
                &[
                    31, 97, 108, 102, 97, 32, 98, 114, 97, 118, 111, 32, 99, 104, 97, 114, 108,
                    105, 101, 32, 100, 101, 108, 116, 97, 32, 101, 99, 104, 111, 32, 102,
                ],
                "[31,97,108,102,97,32,98,114,97,118,111,32,99,104,97,114,108,105,101,32,100,101,\
                 108,116,97,32,101,99,104,111,32,102]",
            ),
            (
                SOME_WORDS[..32].as_bytes(),
                "0x20_A27844C8191F52E18BEDA28DB8A6D33CB7938949E8115B937EF0AE30527715",
                &[
                    32, 162, 120, 68, 200, 25, 31, 82, 225, 139, 237, 162, 141, 184, 166, 211, 60,
                    183, 147, 137, 73, 232, 17, 91, 147, 126, 240, 174, 48, 82, 119, 21,
                ],
                "[32,162,120,68,200,25,31,82,225,139,237,162,141,184,166,211,60,183,147,137,73,\
                 232,17,91,147,126,240,174,48,82,119,21]",
            ),
            (
                SOME_WORDS[..33].as_bytes(),
                "0x21_97485EF44E4280EE7C6746A60C0BE09EBD80EA91E95FA1C0CA7304B1372273",
                &[
                    33, 151, 72, 94, 244, 78, 66, 128, 238, 124, 103, 70, 166, 12, 11, 224, 158,
                    189, 128, 234, 145, 233, 95, 161, 192, 202, 115, 4, 177, 55, 34, 115,
                ],
                "[33,151,72,94,244,78,66,128,238,124,103,70,166,12,11,224,158,189,128,234,145,233,\
                 95,161,192,202,115,4,177,55,34,115]",
            ),
            (
                SOME_WORDS[..].as_bytes(),
                "0x9405_C1C46B79DCBFF80865BF1DEBFE5306C0CD49C01392CAD7458136FBB2A99E",
                &[
                    148, 5, 193, 196, 107, 121, 220, 191, 248, 8, 101, 191, 29, 235, 254, 83, 6,
                    192, 205, 73, 192, 19, 146, 202, 215, 69, 129, 54, 251, 178, 169, 158,
                ],
                "[148,5,193,196,107,121,220,191,248,8,101,191,29,235,254,83,6,192,205,73,192,19,\
                 146,202,215,69,129,54,251,178,169,158]",
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
            assert_eq!(serialized_bytes, serialized);
            let deserialized = postcard::from_bytes(&serialized).unwrap();
            assert_eq!(blob_id, deserialized);
        }
    }

    #[test]
    fn test_blob_id_serialize_as_text() {
        for (bytes, _debug, _serialized_bytes, serialized_text) in samples() {
            let blob_id = BlobId::new(bytes);
            let serialized = serde_json::to_string(&blob_id).unwrap();
            assert_eq!(serialized_text, serialized);
            let deserialized: BlobId = serde_json::from_str(&serialized).unwrap();
            assert_eq!(blob_id, deserialized);
        }
    }
}
