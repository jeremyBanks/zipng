use std::fmt;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::Arc;

use databake::Bake;
use once_cell::sync::OnceCell;
use serde::ser::SerializeTuple;
use serde::Deserialize;
use serde::Serialize;

pub use self::blob_id::BlobId;
use crate::generic::never;
use crate::generic::Ellipses;

mod blob_id;

#[derive(Clone, Bake, Serialize, Deserialize, Default)]
#[databake(path = fiction)]
#[serde(transparent)]
pub struct Blob<Representing: Representable> {
    bytes: Arc<Vec<u8>>,
    #[serde(skip)]
    represented: PhantomData<fn() -> Representing>,
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

impl<Representing: Representable> AsRef<[u8]> for Blob<Representing> {
    fn as_ref(&self) -> &[u8] {
        self.bytes.as_ref()
    }
}

impl<Representing: Representable> From<Vec<u8>> for Blob<Representing> {
    fn from(bytes: Vec<u8>) -> Self {
        Self {
            bytes: Arc::new(bytes),
            ..Default::default()
        }
    }
}

impl<Representing: Representable> Debug for Blob<Representing> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Blob")
            .field("id()", &self.id())
            .field("bytes", &Ellipses)
            .finish()
    }
}

impl<Representing: Representable> Blob<Representing> {
    pub fn new(bytes: impl AsRef<[u8]>) -> Self {
        Self {
            bytes: Arc::new(serde_bytes::ByteBuf::from(bytes.as_ref())),
            represented: OnceCell::new(),
        }
    }

    pub fn id(&self) -> BlobId<Representing> {
        BlobId::from(&self)
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}
