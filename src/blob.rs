#[cfg(test)]
mod test;
mod blob_id;

pub use self::blob_id::BlobId;

use std::fmt;
use std::fmt::Debug;
use std::sync::Arc;

use serde::Deserialize;
use serde::Serialize;

use crate::generic::Ellipses;
use once_cell::sync::OnceCell;
use databake::Bake;

#[derive(Clone, Bake, Serialize, Deserialize, Default)]
#[databake(path = fiction)]
#[serde(transparent)]
pub struct Blob<Representing = ()>
where Representing: Debug + Serialize + Deserialize<'static> + 'static {
    bytes: Arc<serde_bytes::ByteBuf>,
    #[serde(skip)]
    represented: OnceCell<Arc<Representing>>,
    #[serde(skip)]
    blob_id: OnceCell<BlobId<Representing>>,
}

impl From<Vec<u8>> for Blob {
    fn from(bytes: Vec<u8>) -> Self {
        Self {
            bytes: Arc::new(serde_bytes::ByteBuf::from(bytes)),
            ..Default::default()
        }
    }
}

impl Debug for Blob {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Blob")
            .field("id()", &self.id())
            .field("len()", &self.len())
            .field("bytes", &Ellipses)
            .finish()
    }
}

impl<Representing> From<Representing> for Blob<Representing>
where Representing: Debug + Serialize + Deserialize<'static> + 'static {
    fn from(represented: Representing) -> Self {
        Self::new(postcard::to_stdvec(&represented).unwrap())
    }
}

impl<Representing> Blob<Representing>
where Representing: Debug + Serialize + Deserialize<'static> + 'static {
    pub fn new(bytes: impl AsRef<[u8]>) -> Self {
        Self {
            bytes: Arc::new(serde_bytes::ByteBuf::from(bytes.as_ref())),
            blob_id: OnceCell::new(),
            represented: OnceCell::new(),
         }
    }

    pub fn id(&self) -> BlobId {
        BlobId::from_bytes(&self.bytes)
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}
