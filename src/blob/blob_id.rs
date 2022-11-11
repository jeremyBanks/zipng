use std::fmt::Debug;
use std::marker::PhantomData;

use serde::Deserialize;
use serde::Serialize;

use super::Representable;
use crate::never;
use crate::Blob;

#[derive(Default, Debug, Eq, PartialOrd, PartialEq, Ord, Hash, Serialize, Deserialize)]
#[serde(from = "serde_bytes::ByteBuf", into = "serde_bytes::ByteBuf")]
#[serde(bound(serialize = "Representing: Into<Representing>", deserialize = "Representing: Into<Representing>"))]
pub struct BlobId<Representing: Representable + ?Sized> {
    blob_id: heapless::Vec<u8, 32>,
    #[serde(skip)]
    representing: PhantomData<fn() -> Representing>,
}

impl<Representing: Representable> Clone for BlobId<Representing> {
    fn clone(&self) -> Self {
        Self {
            blob_id: self.blob_id.clone(),
            representing: PhantomData,
        }
    }
}

impl<Representing: Representable> BlobId<Representing> {
    pub fn new(slice: impl AsRef<[u8]>) -> BlobId<Representing> {
        let slice = slice.as_ref();
        let blob_id = heapless::Vec::<u8, 32>::new();

        if slice.len() <= blob_id.capacity() {
            blob_id.extend(slice.iter().copied());
        } else {
            blob_id.extend(blake3::hash(slice).as_bytes().iter().copied());
        }

        BlobId {
            blob_id,
            representing: PhantomData,
        }
    }
}

impl<Representing: Representable> AsRef<[u8]> for BlobId<Representing> {
    fn as_ref(&self) -> &[u8] {
        self.blob_id.as_ref()
    }
}

impl<Representing: Representable> From<Blob<Representing>> for BlobId<Representing> {
    fn from(value: Blob<Representing>) -> Self {
        BlobId::new(value)
    }
}

impl<Representing: Representable> From<&Blob<Representing>> for BlobId<Representing> {
    fn from(value: &Blob<Representing>) -> Self {
        BlobId::new(value)
    }
}

impl<Representing: Representable> From<&[u8]> for BlobId<Representing> {
    fn from(value: &[u8]) -> Self {
        Self::from(value.as_ref())
    }
}

impl<Representing: Representable> From<BlobId<Representing>> for serde_bytes::ByteBuf {
    fn from(value: BlobId<Representing>) -> Self {
        Self::from(value.as_ref())
    }
}

impl<Representing: Representable> From<serde_bytes::ByteBuf> for BlobId<Representing> {
    fn from(value: serde_bytes::ByteBuf) -> Self {
        Self::new(value.as_ref())
    }
}
