use std::fmt;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::Arc;

use serde::Deserialize;
use serde::Serialize;

pub use self::blob_id::BlobId;
use crate::generic::Ellipses;
use crate::phantom_type::PhantomType;

mod blob_id;

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct Blob<Representing: Representable + ?Sized> {
    #[serde(with = "serde_bytes")]
    bytes: Vec<u8>,
    #[serde(skip)]
    representing: PhantomType<Representing>,
}

impl<Representing: Representable + ?Sized> Default for Blob<Representing> {
    fn default() -> Self {
        Self {
            bytes: Default::default(),
            representing: PhantomData,
        }
    }
}

impl<Representing: Representable + ?Sized> Clone for Blob<Representing> {
    fn clone(&self) -> Self {
        Self {
            bytes: self.bytes.clone(),
            representing: PhantomData,
        }
    }
}

pub trait Representable: Debug {
    type SerdeAs: Serialize + Deserialize<'static>;
}

impl<T> Representable for T
where
    T: Debug + Serialize + Deserialize<'static>,
{
    type SerdeAs = Self;
}

// raw byte string
impl Representable for [u8] {
    type SerdeAs = crate::serde::UnterminatedBytes;
}

// raw utf-8 string
impl Representable for str {
    type SerdeAs = crate::serde::UnterminatedBytes;
}

impl<Representing: Representable + ?Sized> AsRef<[u8]> for Blob<Representing> {
    fn as_ref(&self) -> &[u8] {
        self.bytes.as_ref()
    }
}

impl<Representing: Representable + ?Sized> From<Vec<u8>> for Blob<Representing> {
    fn from(bytes: Vec<u8>) -> Self {
        Self {
            bytes: Arc::new(bytes),
            ..Default::default()
        }
    }
}

impl<Representing: Representable + ?Sized> Debug for Blob<Representing> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Blob")
            .field("id()", &self.id())
            .field("bytes", &Ellipses)
            .finish()
    }
}
