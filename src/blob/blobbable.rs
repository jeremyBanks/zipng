use serde::de::DeserializeOwned;
use serde::Serialize;

use super::serialization::BlobSerialization;
use crate::blob::Blip;
use crate::blob::Blob;

/// A type that can be serialized to/from a [`Blob`].
///
/// This is automatically implemented for any type that's [`Serialize`] and
/// [`DeserializeOwned`], which are serialized using [`postcard`],
pub trait Blobbable<Serialization>
where Serialization: BlobSerialization
{
    /// Serializes this value into a [`Blob`].
    fn to_blob(&self) -> Blob<Self, Serialization>;

    /// Deserialize a [`Blob`] into this type as an owned value if possible,
    /// else None.
    ///
    /// Either `owned_from_blob` or `borrowed_from_blob` must be implemented
    /// and return a non-`None` value.
    fn owned_from_blob(blob: &Blob<Self, Serialization>) -> Option<Self>
    where Self: Sized {
        None
    }

    /// Deserialize a [`Blob`] into this type as a reference if possible, else
    /// None.
    fn borrowed_from_blob(blob: &Blob<Self, Serialization>) -> Option<&Self> {
        None
    }

    /// Deserializes a [`Blob`] into this type as an owned value.
    fn from_blob(blob: &Blob<Self, Serialization>) -> Self
    where Self: Clone {
        Self::owned_from_blob(blob).unwrap_or(
            Self::borrowed_from_blob(&blob)
                .expect("either blob_to or blob_as must be implemented")
                .clone(),
        )
    }

    /// Serialize this value into a [`Blob`] and then return its [`Blip`].
    fn to_blip(&self) -> Blip<Self, Serialization> {
        self.to_blob().blip()
    }
}

impl<Serialization> Blobbable<Serialization> for str
where Serialization: BlobSerialization
{
    fn to_blob(&self) -> Blob<Self, Serialization> {
        Blob::from_raw_bytes(self.as_ref())
    }

    fn borrowed_from_blob(blob: &Blob<Self, Serialization>) -> Option<&Self> {
        std::str::from_utf8(blob.as_ref()).unwrap().into()
    }
}

impl<Serialization> Blobbable<Serialization> for [u8]
where Serialization: BlobSerialization
{
    fn to_blob(&self) -> Blob<Self, Serialization> {
        Blob::from_raw_bytes(self.as_ref())
    }

    fn borrowed_from_blob(blob: &Blob<Self, Serialization>) -> Option<&Self> {
        blob.as_ref().into()
    }
}

impl<Serialization, T> Blobbable<Serialization> for T
where
    T: Serialize + DeserializeOwned,
    Serialization: BlobSerialization,
{
    fn to_blob(&self) -> Blob<Self, Serialization> {
        Serialization::serialize_as_blob(self)
    }

    fn owned_from_blob(blob: &Blob<Self, Serialization>) -> Option<Self> {
        Some(Serialization::deserialize_from_blob(blob))
    }
}
