use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::Blob;
use crate::Blobbable;

/// The default serialization used by Blob unless explicitly indicated otherwise.
pub type DefaultBlobSerialization = Postcard;

/// A [`serde`] implementation that can be used to serialize a [`Blobbable`]
/// into a [`Blob`]/[`Blip`].
pub trait BlobSerialization: Sized {
    fn serialize_as_blob<T: Serialize + Blobbable<Self>>(value: &T) -> Blob<T, Self> {
        Blob::<[u8], Self>::from_raw_bytes(&Self::serialize_as_bytes(value)).retype()
    }
    fn serialize_as_bytes<T: Serialize + Blobbable<Self>>(value: &T) -> Vec<u8>;
    fn deserialize_from_blob<T: DeserializeOwned + Blobbable<Self>>(value: &Blob<T, Self>) -> T {
        Self::deserialize_from_bytes(value.as_ref())
    }
    fn deserialize_from_bytes<T: DeserializeOwned + Blobbable<Self>>(value: &[u8]) -> T;
}

/// [`postcard`]
pub enum Postcard {}
impl BlobSerialization for Postcard {
    fn serialize_as_bytes<T: Serialize + Blobbable<Self>>(value: &T) -> Vec<u8> {
        postcard::to_allocvec(value).unwrap()
    }
    fn deserialize_from_bytes<T: DeserializeOwned + Blobbable<Self>>(value: &[u8]) -> T {
        postcard::from_bytes(value).unwrap()
    }
}

/// [`serde_json`]
pub enum Json {}
impl BlobSerialization for Json {
    fn serialize_as_bytes<T: Serialize + Blobbable<Self>>(value: &T) -> Vec<u8> {
        serde_json::to_vec(value).unwrap()
    }
    fn deserialize_from_bytes<T: DeserializeOwned + Blobbable<Self>>(value: &[u8]) -> T {
        serde_json::from_slice(value).unwrap()
    }
}

/// [`flexbuffers`]
pub enum FlexBuffers {}
impl BlobSerialization for FlexBuffers {
    fn serialize_as_bytes<T: Serialize + Blobbable<Self>>(value: &T) -> Vec<u8> {
        flexbuffers::to_vec(value).unwrap()
    }
    fn deserialize_from_bytes<T: DeserializeOwned + Blobbable<Self>>(value: &[u8]) -> T {
        flexbuffers::from_slice(value).unwrap()
    }
}

/// [`serde_cbor`]
pub enum Cbor {}
impl BlobSerialization for Cbor {
    fn serialize_as_bytes<T: Serialize + Blobbable<Self>>(value: &T) -> Vec<u8> {
        serde_cbor::to_vec(value).unwrap()
    }
    fn deserialize_from_bytes<T: DeserializeOwned + Blobbable<Self>>(value: &[u8]) -> T {
        serde_cbor::from_slice(value).unwrap()
    }
}
