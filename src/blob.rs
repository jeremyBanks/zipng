mod blip;
use std::borrow::Borrow;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::sync::Arc;

use bstr::BStr;
use serde::Deserialize;
use serde::Serialize;

pub use self::blip::Blip;
use crate::generic::PhantomType;

/// A blob is a byte vector, representing the Postcard binary serialization of a
/// value of a given type, or else a raw byte or character string (`Blob<[u8]>`,
/// `Blob<str>`).
pub struct Blob<T>
where
    T: ?Sized,
{
    bytes: Vec<u8>,
    representing: PhantomType<T>,
}

impl<T> Debug for Blob<T>
where
    T: ?Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(BStr::new(&self.bytes), f)
    }
}

impl<T> Display for Blob<T>
where
    T: ?Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(BStr::new(&self.bytes), f)
    }
}

impl<T> Serialize for Blob<T>
where
    T: ?Sized,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serde_bytes::Bytes::new(&self.bytes).serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for Blob<T>
where
    T: ?Sized,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        serde_bytes::ByteBuf::deserialize(deserializer).map(|b| Self::from(b.into_vec()))
    }
}

impl<T> Default for Blob<T>
where
    T: ?Sized,
{
    fn default() -> Self {
        Self {
            bytes: Default::default(),
            representing: PhantomType::new(),
        }
    }
}

impl<T> Clone for Blob<T>
where
    T: ?Sized,
{
    fn clone(&self) -> Self {
        Self {
            bytes: self.bytes.clone(),
            representing: PhantomType::new(),
        }
    }
}
