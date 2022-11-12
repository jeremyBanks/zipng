use std::borrow::Borrow;
use std::convert::Infallible;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;
use std::ops::Deref;
use std::ops::DerefMut;
use std::rc::Rc;

use bstr::BStr;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;
use static_assertions::assert_impl_all;

use super::BlobSerialization;
use super::Blobbable;
use super::DefaultBlobSerialization;
use crate::generic::default;
use crate::generic::Type;
use crate::never;

/// A blob is a byte vector, containing the binary serialization of a
/// value of a given type, or else a raw byte or character string (`Blob<[u8]>`,
/// `Blob<str>`).
pub struct Blob<T, S = DefaultBlobSerialization>
where
    T: ?Sized,
    S: BlobSerialization,
{
    bytes: Vec<u8>,
    t: Type<T>,
    s: Type<S>,
}

assert_impl_all!(Blob<never>: Sized, Serialize, DeserializeOwned, Sync, Send);
assert_impl_all!(Blob<Infallible>: Sized, Serialize, DeserializeOwned, Sync, Send);
assert_impl_all!(Blob<Rc<u8>>: Sized, Serialize, DeserializeOwned, Sync, Send);
assert_impl_all!(Blob<[u8]>: Sized, Serialize, DeserializeOwned, Sync, Send);
assert_impl_all!(Blob<dyn Debug>: Sized, Serialize, DeserializeOwned, Sync, Send);

impl<T, S> Blob<T, S>
where
    T: ?Sized,
    S: BlobSerialization,
{
    /// Creates a new blob from a reference to any blobbable value.
    pub fn new<Ref>(value: Ref) -> Self
    where
        T: Blobbable<S>,
        Ref: Sized + Borrow<T>,
    {
        T::to_blob(value.borrow())
    }

    /// Creates a blob from bytes directly, without making sure the type
    /// matches.
    ///
    /// This is memory-safe, but might cause a panic if you use the wrong type.
    pub fn from_raw_bytes(bytes: &[u8]) -> Self {
        Self::from_raw_vec(bytes.to_vec())
    }

    /// Creates a blob from bytes directly, without making sure the type
    /// matches.
    ///
    /// This is memory-safe, but might cause a panic if you use the wrong type.
    pub fn from_raw_vec(bytes: Vec<u8>) -> Self {
        Self { bytes, ..default() }
    }

    pub(in crate::blob) fn retype<R: ?Sized, Q: BlobSerialization>(self) -> Blob<R, Q> {
        Blob {
            bytes: self.bytes,
            ..default()
        }
    }
}

impl<T, S> Hash for Blob<T, S>
where
    T: ?Sized,
    S: BlobSerialization,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.bytes.hash(state);
    }
}

impl<T, S> PartialEq for Blob<T, S>
where
    T: ?Sized,
    S: BlobSerialization,
{
    fn eq(&self, other: &Self) -> bool {
        self.bytes.eq(&other.bytes)
    }
}

impl<T, S> Eq for Blob<T, S>
where
    T: ?Sized,
    S: BlobSerialization,
{
}

impl<T, S> PartialOrd for Blob<T, S>
where
    T: ?Sized,
    S: BlobSerialization,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.bytes.partial_cmp(&other.bytes)
    }
}

impl<T, S> Ord for Blob<T, S>
where
    T: ?Sized,
    S: BlobSerialization,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.bytes.cmp(&other.bytes)
    }
}

impl<T, S> Debug for Blob<T, S>
where
    T: ?Sized,
    S: BlobSerialization,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(BStr::new(&self.bytes), f)
    }
}

impl<T, S> Display for Blob<T, S>
where
    T: ?Sized,
    S: BlobSerialization,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(BStr::new(&self.bytes), f)
    }
}

impl<T, S> Serialize for Blob<T, S>
where
    T: ?Sized,
    S: BlobSerialization,
{
    fn serialize<Ser>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error>
    where Ser: serde::Serializer {
        serde_bytes::Bytes::new(self.bytes.as_ref()).serialize(serializer)
    }
}

impl<'de, T, S> Deserialize<'de> for Blob<T, S>
where
    T: ?Sized,
    S: BlobSerialization,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let bytes = serde_bytes::ByteBuf::deserialize(deserializer)?.into_vec();
        Ok(Blobbable::<S>::to_blob(&bytes).retype())
    }
}

impl<T, S> Default for Blob<T, S>
where
    T: ?Sized,
    S: BlobSerialization,
{
    fn default() -> Self {
        Self {
            bytes: default(),
            t: default(),
            s: default(),
        }
    }
}

impl<T, S> Clone for Blob<T, S>
where
    T: ?Sized,
    S: BlobSerialization,
{
    fn clone(&self) -> Self {
        Self {
            bytes: self.bytes.clone(),
            ..default()
        }
    }
}

impl<T, S> AsRef<[u8]> for Blob<T, S>
where
    T: ?Sized,
    S: BlobSerialization,
{
    fn as_ref(&self) -> &[u8] {
        self.bytes.as_ref()
    }
}

impl<T, S> AsMut<[u8]> for Blob<T, S>
where
    T: ?Sized,
    S: BlobSerialization,
{
    fn as_mut(&mut self) -> &mut [u8] {
        self.bytes.as_mut()
    }
}

impl<T, S> Deref for Blob<T, S>
where
    T: ?Sized,
    S: BlobSerialization,
{
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        self.as_ref()
    }
}

impl<T, S> DerefMut for Blob<T, S>
where
    T: ?Sized,
    S: BlobSerialization,
{
    fn deref_mut(&mut self) -> &mut [u8] {
        self.as_mut()
    }
}

impl<T, S> PartialEq<T> for Blob<T, S>
where
    T: ?Sized + Blobbable<S>,
    S: BlobSerialization,
{
    fn eq(&self, other: &T) -> bool {
        self == &other.to_blob()
    }
}
