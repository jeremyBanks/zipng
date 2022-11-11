use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;

use bstr::BStr;
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;

use crate::blob::Blob;
use crate::generic::default;
use crate::generic::PhantomType;

/// A `Blip` represents a `Blob`, stored inline if it's under 32 bytes,
/// otherwise represented by its 32-byte BLAKE3 hash digest.
pub struct Blip<T>
where
    T: ?Sized,
{
    bytes: heapless::Vec<u8, 32>,
    representing: PhantomType<T>,
}

// impl<T> Copy for Blip<T> where T: ?Sized {}

#[derive(Debug, Error, Copy, Clone)]
#[error("Blips be between 0 and 32 bytes, but the input was {0} bytes.")]
pub struct TooLongForBlipError(usize);

impl<T> Blip<T>
where
    T: ?Sized,
{
    pub fn inline(&self) -> Option<Blob<T>> {
        if self.bytes.len() < 32 {
            Some(Blob::for_bytes(&self.bytes))
        } else {
            None
        }
    }

    fn try_from_raw_bytes(blip_bytes: &[u8]) -> Result<Self, TooLongForBlipError> {
        Ok(Self {
            bytes: heapless::Vec::from_slice(&blip_bytes)
                .map_err(|_| TooLongForBlipError(blip_bytes.len()))?,
            representing: default(),
        })
    }

    fn retype<R: ?Sized>(self) -> Blip<R> {
        Blip {
            bytes: self.bytes,
            representing: default(),
        }
    }
}

impl<T> From<Blob<T>> for Blip<T>
where
    T: ?Sized,
{
    fn from(value: Blob<T>) -> Self {
        Blip::for_bytes(value.as_ref()).retype()
    }
}

#[derive(Debug, Error, Copy, Clone)]
#[error("This Blip represents a value that's too long to store inline ({0} bytes > 31 bytes).")]
pub struct TooLongForInlineBlipError(usize);

impl<T> TryFrom<Blip<T>> for Blob<T>
where
    T: ?Sized,
{
    type Error = TooLongForInlineBlipError;

    fn try_from(value: Blip<T>) -> Result<Self, Self::Error> {}
}

impl Blip<[u8]> {
    pub fn for_bytes(mut bytes: &[u8]) -> Self {
        if bytes.len() >= 32 {
            bytes = blake3::hash(bytes).as_bytes()
        }
        Self {
            bytes: heapless::Vec::from_slice(bytes).unwrap(),
            representing: default(),
        }
    }
}

impl Blip<str> {
    pub fn for_str(bytes: &str) -> Self {
        Blip::for_bytes(bytes.as_bytes()).retype()
    }
}

impl<T> Blip<T>
where
    T: Serialize + ?Sized,
{
    pub fn for_value(value: &T) -> Self {
        Blip::for_bytes(&postcard::to_allocvec(value).expect("serialization must not fail"))
            .retype()
    }
}

impl<T> Blip<T>
where
    T: ?Sized,
{
    pub fn blob(value: &Blob<T>) -> Self {
        Blip::for_bytes(value.as_ref()).retype()
    }
}

impl<T> Hash for Blip<T>
where
    T: ?Sized,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.bytes.hash(state);
    }
}

impl<T> PartialEq for Blip<T>
where
    T: ?Sized,
{
    fn eq(&self, other: &Self) -> bool {
        self.bytes.eq(&other.bytes)
    }
}

impl<T> Eq for Blip<T> where T: ?Sized {}

impl<T> PartialOrd for Blip<T>
where
    T: ?Sized,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.bytes.partial_cmp(&other.bytes)
    }
}

impl<T> Ord for Blip<T>
where
    T: ?Sized,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.bytes.cmp(&other.bytes)
    }
}

impl<T> Debug for Blip<T>
where
    T: ?Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(BStr::new(&self.bytes), f)
    }
}

impl<T> Display for Blip<T>
where
    T: ?Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(BStr::new(&self.bytes), f)
    }
}

impl<T> Serialize for Blip<T>
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

impl<'de, T> Deserialize<'de> for Blip<T>
where
    T: ?Sized,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bytes = serde_bytes::ByteBuf::deserialize(deserializer)?.into_vec();
        Self::try_from_raw_bytes(&bytes).map_err(|err| serde::de::Error::custom(err.to_string()))
    }
}

impl<T> Default for Blip<T>
where
    T: ?Sized,
{
    fn default() -> Self {
        Self {
            bytes: default(),
            representing: default(),
        }
    }
}

impl<T> Clone for Blip<T>
where
    T: ?Sized,
{
    fn clone(&self) -> Self {
        Self {
            bytes: self.bytes.clone(),
            representing: default(),
        }
    }
}

impl<T> AsRef<[u8]> for Blip<T>
where
    T: ?Sized,
{
    fn as_ref(&self) -> &[u8] {
        self.bytes.as_ref()
    }
}
