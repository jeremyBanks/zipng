use std::sync::Arc;

use crate::blob::Blob;
use crate::blob::BlobId;
use crate::storage::Storage;

#[derive(Debug, Default)]
pub struct Context {
    storage: Option<Arc<Storage>>,
}

impl Context {
    pub fn new(storage: impl Into<Option<Arc<Storage>>>) -> Context {
        let storage = storage.into();
        Context { storage }
    }

    pub fn get_blob(&self, id: BlobId) -> Option<Blob> {
        None
    }

    pub fn insert_blob(&mut self, data: impl Into<Blob>) {}
}

// Okay
// I need content-addressed, of course
// but I also need a (versioned) K-V on top of it that can be used
// directly, for inferred non-canonical word snippets or whatever
// and maybe those should be defined in terms of slice operations?
//
// Okay, no -- generic KV isn't quite it, we just want to be able to
// insert synthesized results into the cache, for queries that aren't
// actually possible to execute.
//
// So this is okay. But first make some useful TTS output.
//
// Do a "SPIKE"
