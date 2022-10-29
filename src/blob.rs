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

#[derive(Default, Clone, Copy, Eq, PartialOrd, PartialEq, Ord, Hash)]
pub struct BlobId {
    inline: [u8; BlobId::INLINE],
    length: u32,
}

impl Debug for BlobId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(inline) = self.as_inline() {
            write!(f, "{:?}", BStr::new(inline))
        } else {
            write!(f, "0x{}", hex::encode_upper(self.to_serialized_bytes()))
        }
    }
}

impl BlobId {
    pub(crate) const INLINE: usize = 28;

    pub fn new(length: usize, inline: [u8; BlobId::INLINE]) -> BlobId {
        let length32 = length.try_into().unwrap();
        if length < BlobId::INLINE {
            if inline[length..].iter().any(|&b| b != 0x00) {
                panic!("inline padding must be zeroed");
            }
        }
        BlobId {
            length: length32,
            inline,
        }
    }

    pub fn len(&self) -> usize {
        self.length as usize
    }

    fn inline(&self) -> &[u8] {
        &self.inline[..self.len().min(BlobId::INLINE)]
    }

    pub fn is_inline(&self) -> bool {
        self.len() <= BlobId::INLINE
    }

    pub fn as_inline(&self) -> Option<&[u8]> {
        if self.is_inline() {
            Some(&self.inline[..self.len()])
        } else {
            None
        }
    }

    pub fn is_hash(&self) -> bool {
        self.len() > BlobId::INLINE
    }

    pub fn as_hash(&self) -> Option<&[u8; BlobId::INLINE]> {
        if self.is_hash() {
            Some(&self.inline)
        } else {
            None
        }
    }

    pub fn for_bytes(slice: impl AsRef<[u8]>) -> BlobId {
        let slice = slice.as_ref();
        let length = slice.len();
        let mut inline = [0x00; BlobId::INLINE];
        if length > BlobId::INLINE {
            let hash = blake3::hash(slice);
            inline.copy_from_slice(&hash.as_bytes()[..BlobId::INLINE]);
        } else {
            inline[..length].copy_from_slice(slice);
        }
        BlobId::new(length, inline)
    }

    pub fn to_serialized_bytes(&self) -> heapless::Vec<u8, 32> {
        postcard::to_vec(&self).expect("infallible")
    }

    pub fn from_serialized_bytes(slice: impl AsRef<[u8]>) -> Result<BlobId, postcard::Error> {
        postcard::from_bytes(slice.as_ref())
    }

    pub fn to_serialized_text(&self) -> String {
        serde_json::to_string(&self).expect("infallible")
    }

    pub fn from_serialized_text(slice: impl AsRef<[u8]>) -> Result<BlobId, serde_json::Error> {
        serde_json::from_slice(slice.as_ref())
    }
}

impl Serialize for BlobId {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        return serializer.serialize_tuple(2).and_then(|mut s| {
            s.serialize_element(&self.length)?;
            s.serialize_element(&SerializeInlineBytes(self))?;
            s.end()
        });

        struct SerializeInlineBytes<'a>(&'a BlobId);
        impl Serialize for SerializeInlineBytes<'_> {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer
                    .serialize_tuple(self.0.len().min(BlobId::INLINE))
                    .and_then(|mut s| {
                        for &b in self.0.inline() {
                            s.serialize_element(&b)?;
                        }
                        s.end()
                    })
            }
        }
    }
}

impl<'input> Deserialize<'input> for BlobId {
    fn deserialize<D: Deserializer<'input>>(deserializer: D) -> Result<Self, D::Error> {
        return deserializer.deserialize_tuple(2, BlobIdVisitor);

        struct BlobIdVisitor;
        impl<'input> Visitor<'input> for BlobIdVisitor {
            type Value = BlobId;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a tuple of (u32, (u8, u8...))")
            }

            fn visit_seq<A: SeqAccess<'input>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let length = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;

                macro_rules! fml {
                    () => {
                        fml![
                            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
                            16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28
                        ]
                    };
                    [$($i:expr),+] => {$(
                    if length.min(BlobId::INLINE) == $i {
                        let inline: BlobIdInlineDeserializer<$i> = seq
                        .next_element()?
                        .ok_or_else(|| de::Error::invalid_length(length, &self))?;
                        inline.0
                    } else
                )+ { unreachable!() }} }

                Ok(BlobId::new(length, fml!()))
            }
        }

        struct BlobIdInlineDeserializer<const LENGTH: usize>([u8; BlobId::INLINE]);
        impl<'input, const LENGTH: usize> Deserialize<'input> for BlobIdInlineDeserializer<LENGTH> {
            fn deserialize<D: Deserializer<'input>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_tuple(LENGTH, BlobIdInlineVisitor::<LENGTH>)
            }
        }

        struct BlobIdInlineVisitor<const LENGTH: usize>;
        impl<'input, const LENGTH: usize> Visitor<'input> for BlobIdInlineVisitor<LENGTH> {
            type Value = BlobIdInlineDeserializer<LENGTH>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "a tuple of {LENGTH} bytes")
            }

            fn visit_seq<A: SeqAccess<'input>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let mut inline = [0x00; BlobId::INLINE];
                for i in 0..LENGTH {
                    inline[i] = seq
                        .next_element()?
                        .ok_or_else(|| de::Error::invalid_length(LENGTH, &self))?;
                }
                Ok(BlobIdInlineDeserializer(inline))
            }
        }
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
            let blob_id = BlobId::for_bytes(bytes);
            assert_eq!(format!("{blob_id:?}"), debug);
        }
    }

    #[test]
    fn test_blob_id_serialize() {
        for (bytes, _debug, serialized_bytes, _serialized_text) in samples() {
            let blob_id = BlobId::for_bytes(bytes);
            let serialized = blob_id.to_serialized_bytes();
            let deserialized = BlobId::from_serialized_bytes(&serialized).unwrap();
            assert_eq!(blob_id, deserialized);
            assert_eq!(serialized_bytes, serialized);
        }
    }

    #[test]
    fn test_blob_id_serialize_as_text() {
        for (bytes, _debug, _serialized_bytes, serialized_text) in samples() {
            let blob_id = BlobId::for_bytes(bytes);
            let serialized = blob_id.to_serialized_text();
            let deserialized = BlobId::from_serialized_text(&serialized).unwrap();
            assert_eq!(blob_id, deserialized);
            assert_eq!(serialized_text, serialized);
        }
    }
}
