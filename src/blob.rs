use std::fmt;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::Arc;

use databake::Bake;
use once_cell::sync::OnceCell;
use serde::Deserialize;
use serde::Serialize;

pub use self::blob_id::BlobId;
use crate::generic::never;
use crate::generic::Ellipses;

mod blob_id;

#[derive(Clone, Bake, Serialize, Deserialize, Default)]
#[databake(path = fiction)]
#[serde(transparent)]
pub struct Blob<Representing>
where
    Representing: Debug + Serialize + Deserialize<'static> + 'static,
{
    bytes:       Arc<Vec<u8>>,
    #[serde(skip)]
    represented: PhantomData<fn() -> Representing>,
}

impl<Representing> AsRef<[u8]> for Blob<Representing>
where
    Representing: Debug + Serialize + Deserialize<'static> + 'static,
{
    fn as_ref(&self) -> &[u8] {
        self.bytes.as_ref()
    }
}

impl<Representing> From<Vec<u8>> for Blob<Representing>
where
    Representing: Debug + Serialize + Deserialize<'static> + 'static,
{
    fn from(bytes: Vec<u8>) -> Self {
        Self {
            bytes: Arc::new(bytes),
            ..Default::default()
        }
    }
}

impl<Representing> Debug for Blob<Representing>
where
    Representing: Debug + Serialize + Deserialize<'static> + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Blob")
            .field("id()", &self.id())
            .field("bytes", &Ellipses)
            .finish()
    }
}

impl<Representing> Blob<Representing>
where
    Representing: Debug + Serialize + Deserialize<'static> + 'static,
{
    pub fn new(bytes: impl AsRef<[u8]>) -> Self {
        Self {
            bytes:       Arc::new(serde_bytes::ByteBuf::from(bytes.as_ref())),
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
