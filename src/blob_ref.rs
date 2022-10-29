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
use serde::ser::SerializeTuple;
use serde::de::Visitor;
use serde::de;
use serde::de::SeqAccess;

#[derive(Default, Debug, Clone, Copy, Eq, PartialOrd, PartialEq, Ord, Hash)]
pub struct BlobId {
    inline: [u8; BlobId::INLINE],
    length: u32,
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
        BlobId { length: length32, inline }
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
        if length >= BlobId::INLINE {
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
}

#[test]
fn test_serialize_blob_id() {
    let blob_id = BlobId::for_bytes(b"hello world");
    println!("{:02X?}", &blob_id);
    let serialized = blob_id.to_serialized_bytes();
    println!("{:02X?}", &serialized);
    let deserialized = BlobId::from_serialized_bytes(&serialized).unwrap();
    println!("{:02X?}", &deserialized);
    assert_eq!(blob_id, deserialized);
}

impl Serialize for BlobId {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        return serializer.serialize_tuple(2).and_then(|mut s| {
            s.serialize_element(&self.length)?;
            s.serialize_element(&SerializeElement(self.length, self.inline()))?;
            s.end()
        });

        struct SerializeElement<'a>(u32, &'a [u8]);
        impl Serialize for SerializeElement<'_> {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_tuple(self.0 as usize).and_then(|mut s| {
                    for &b in self.1 {
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
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                if length == 0 {
                    let inline: () = seq
                        .next_element()?
                        .ok_or_else(|| de::Error::invalid_length(length, &self))?;
                } else if length == 1 {
                    let inline: u8 = seq
                        .next_element()?
                        .ok_or_else(|| de::Error::invalid_length(length, &self))?;
                } else if length == 2 {
                    let inline: (u8, u8) = seq
                        .next_element()?
                        .ok_or_else(|| de::Error::invalid_length(length, &self))?;
                }
                let inline = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                Ok(BlobId::new(length, inline))
            }
        }

        // struct BlobIdElementVisitor(usize);
        // impl<'input> Visitor<'input> for BlobIdElementVisitor {
        //     type Value = [u8; BlobId::INLINE];

        //     fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        //         write!(formatter, "a tuple of {} bytes", self.0)
        //     }

        //     fn visit_seq<A: SeqAccess<'input>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        //         let mut inline = [0x00; BlobId::INLINE];
        //         for i in 0..self.0 {
        //             inline[i] = seq
        //                 .next_element()?
        //                 .ok_or_else(|| de::Error::invalid_length(self.0, &self))?;
        //         }
        //         Ok(inline)
        //     }
        // }

    }
}


struct U32Visitor;
impl<'input> serde::de::Visitor<'input> for U32Visitor {
    type Value = u32;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("u32")
    }

    fn visit_u32<E: serde::de::Error>(self, value: u32) -> Result<Self::Value, E> {
        Ok(value)
    }
}

