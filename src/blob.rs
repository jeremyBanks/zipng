#[cfg(test)]
mod test;

use std::collections::HashMap;
use std::convert::Infallible;
use std::env;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::format as f;
use std::hash::Hasher;
use std::ops::Deref;
use std::str;
use std::sync::Arc;

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
use crate::generic::Ellipses;

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Blob {
    bytes: Arc<[u8]>,
    id: BlobId,
}

impl Debug for Blob {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Blob")
            .field("id", &self.id())
            .field("bytes", &Ellipses)
            .field("len()", &self.len())
            .finish()
    }
}

impl Blob {
    pub fn new(bytes: impl Into<Arc<[u8]>>) -> Self {
        let bytes = bytes.into();
        let id = BlobId::new(&bytes);
        Self { id, bytes }
    }

    pub fn id(&self) -> BlobId {
        self.id
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl<T> From<T> for Blob
where
    T: Into<Arc<[u8]>>,
{
    fn from(bytes: T) -> Self {
        Self::new(bytes)
    }
}

impl From<&Blob> for BlobId {
    fn from(blob: &Blob) -> Self {
        blob.id
    }
}

impl From<Blob> for BlobId {
    fn from(blob: Blob) -> Self {
        blob.id
    }
}

impl FromIterator<u8> for Blob {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        Self::new(iter.into_iter().collect::<Vec<_>>())
    }
}

impl<'a> FromIterator<&'a u8> for Blob {
    fn from_iter<T: IntoIterator<Item = &'a u8>>(iter: T) -> Self {
        Self::new(iter.into_iter().copied().collect::<Vec<_>>())
    }
}

impl Deref for Blob {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}

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
        // serializer.is_human_readable();
        // <-- this is how we make sure both JSON and Postcard are sane
        // use the to_string hex-like encoding, add a FromStr impl
        // and use that for deserialization.
        let buffer_len = 1 + BlobId::BUFFER - self.len_len();
        let mut tuple = serializer.serialize_tuple(buffer_len)?;
        tuple.serialize_element(&self.len())?;
        for byte in &self.buffer[self.len_len()..(self.len_len() + self.len()).min(BlobId::BUFFER)]
        {
            tuple.serialize_element(&*byte)?;
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
