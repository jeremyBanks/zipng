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

const HASH_LENGTH: usize = blake3::OUT_LEN;
const MAX_INLINE_LENGTH: usize = HASH_LENGTH - 1;
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]]
pub struct BlobRef {
    bytes: [u8; HASH_LENGTH],
    length: usize,
}

impl From<BlobRef> for heapless::Vec::<u8, 8> {

}

pub fn blob(bytes: impl AsRef<[u8]>) -> BlobRef {
    BlobRef::from_bytes(bytes)
}

impl BlobRef {
    pub fn from_bytes(slice: impl AsRef<[u8]>) -> BlobRef {
        let slice = slice.as_ref();
        let slice_length = slice.len();
        let mut bytes = [0u8; blake3::OUT_LEN];
        if slice_length >= blake3::OUT_LEN {
            let hash = blake3::hash(slice);
            bytes.copy_from_slice(hash.as_bytes());
        } else {
            bytes[..slice_length].copy_from_slice(slice);
        }
        let length = slice_length.min(32);
        Self { length, bytes }
    }

    pub fn to_ref(&self) -> Vec<u8> {
        self.as_ref().to_vec()
    }

    pub fn from_ref(slice: impl AsRef<[u8]>) -> Self {
        assert!(slice.as_ref().len() <= HASH_LENGTH);
        let slice = slice.as_ref();
        let mut bytes = [0; HASH_LENGTH];
        bytes[..slice.len()].copy_from_slice(slice);
        Self {
            bytes,
            length: slice.len(),
        }
    }

    pub fn as_inline(&self) -> Option<&[u8]> {
        if self.length < blake3::OUT_LEN {
            Some(self.as_ref())
        } else {
            None
        }
    }
}

impl AsRef<[u8]> for BlobRef {
    fn as_ref(&self) -> &[u8] {
        &self.bytes[..self.length]
    }
}

impl Display for BlobRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        Ok(if let Some(inline) = self.as_inline() {
            for byte in self.as_ref() {
                f.write_char(char::from(*byte));
            }
        } else {
            for byte in self.as_ref() {
                write!(f, "{byte:02X}")?;
            }
        })
    }
}

impl FromStr for BlobRef {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.len() {
            blake3::OUT_LEN => BlobRef::new(&hex::decode(s)?),
            0..=31 => BlobRef::new(s.as_bytes()),
            len => bail!("impossible blob length {len:?}"),
        })
    }
}

impl Debug for BlobRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        Ok(Debug::fmt(&Display::fmt(self, f)?, f)?)
    }
}

impl Serialize for BlobRef {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

use std::fmt::Write;
use std::str::FromStr;

use eyre::bail;
use std::ops::Deref;

impl<'input> Deserialize<'input> for BlobRef {
    fn deserialize<D: Deserializer<'input>>(deserializer: D) -> Result<Self, D::Error> {
        return deserializer.deserialize_str(Visitor);

        struct Visitor;
        impl<'input> serde::de::Visitor<'input> for Visitor {
            type Value = BlobRef;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str(
                    "a string up to 31 characters up to \\xFF, or 64 hex digits, or up to 32  \
                     bytes",
                )
            }

            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                v.parse().map_err(serde::de::Error::custom)
            }

            fn visit_bytes<E: serde::de::Error>(self, v: &[u8]) -> Result<Self::Value, E> {
                Ok(BlobRef::new(v))
            }
        }
    }
}
