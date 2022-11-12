use serde::Deserialize;
use serde::Serialize;

use crate::blob::Blip;
use crate::blob::Blob;
use crate::generic::Borrowable;

/// A type that can be serialized to/from a [`Blob`].
///
/// This is automatically implemented for any type that's [`Serialize`] and
/// [`Deserialize<'static>`], which are serialized using [`postcard`],
pub trait Blobbable {
    fn blob(&self) -> Blob<Self>;

    fn blob_to(blob: &Blob<Self>) -> Borrowable<Self>
    where Self: Sized;

    fn blip(&self) -> Blip<Self> {
        self.blob().blip()
    }
}

impl Blobbable for str {
    fn blob(&self) -> Blob<Self> {
        Blob::from_raw_bytes(self.as_ref())
    }

    fn blob_to(blob: &Blob<Self>) -> Borrowable<Self> {
        std::str::from_utf8(blob.as_ref()).unwrap().into()
    }
}

impl Blobbable for [u8] {
    fn blob(&self) -> Blob<Self> {
        Blob::from_raw_bytes(self.as_ref())
    }

    fn blob_to(blob: &Blob<Self>) -> Borrowable<Self>
    where Self: Sized {
        blob.as_ref().into()
    }
}

impl<T> Blobbable for T
where T: Serialize + Deserialize<'static> + 'static
{
    fn blob(&self) -> Blob<Self> {
        Blob::from_raw_vec(postcard::to_stdvec(self).unwrap())
    }

    fn blob_to(blob: &Blob<Self>) -> Borrowable<Self>
    where Self: Sized {
        postcard::from_bytes(blob.as_ref()).unwrap().into()
    }
}
