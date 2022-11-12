use std::borrow::Borrow;
use std::borrow::BorrowMut;

use derive_more::From;
use derive_more::TryInto;
use serde::Deserialize;
use serde::Serialize;

use crate::blob::Blip;
use crate::blob::Blob;

/// A type that can be serialized to/from a [`Blob`].
///
/// This is automatically implemented for any type that's [`Serialize`] and
/// [`Deserialize<'static>`], which are serialized using [`postcard`],
pub trait Blobbable {
    fn blob(&self) -> Blob<Self>;

    type BlobAs: ?Sized;
    fn blob_as(blob: &Blob<Self>) -> &Self::BlobAs;

    type BlobTo: Sized;
    fn blob_to(blob: &Blob<Self>) -> Self::BlobTo;

    fn blip(&self) -> Blip<Self> {
        self.blob().blip()
    }
}

impl Blobbable for str {
    fn blob(&self) -> Blob<Self> {
        Blob::from_raw_bytes(self.as_ref())
    }

    type BlobAs = str;
    fn blob_as(blob: &Blob<Self>) -> &Self::BlobAs {
        std::str::from_utf8(blob.as_ref()).unwrap()
    }

    type BlobTo = ();
    fn blob_to(blob: &Blob<Self>) {}
}

impl Blobbable for [u8] {
    fn blob(&self) -> Blob<Self> {
        Blob::from_raw_bytes(self.as_ref())
    }

    type BlobAs = [u8];
    fn blob_as(blob: &Blob<Self>) -> &Self::BlobAs {
        blob.as_ref()
    }

    type BlobTo = ();
    fn blob_to(blob: &Blob<Self>) -> Self::BlobTo {}
}

impl<T> Blobbable for T
where T: Serialize + Deserialize<'static> + 'static
{
    fn blob(&self) -> Blob<Self> {
        Blob::from_raw_vec(postcard::to_stdvec(self).unwrap())
    }

    type BlobAs = ();
    fn blob_as(blob: &Blob<Self>) -> &Self::BlobAs {
        &()
    }

    type BlobTo = T;
    fn blob_to(blob: &Blob<Self>) -> Self::BlobTo {
        postcard::from_bytes(blob.as_ref()).unwrap()
    }
}
