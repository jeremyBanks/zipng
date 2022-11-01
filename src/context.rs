use std::error::Error;
use std::sync::Arc;

use derive_more::Deref;
use derive_more::DerefMut;
use derive_more::From;
use miette::Diagnostic;
use thiserror::Error;

use crate::blob::Blob;
use crate::blob::BlobId;
use crate::generic::never;
use crate::queries::Request;
use crate::queries::Response;
use crate::storage::Storage;

#[derive(Debug, Default)]
pub struct Context {
    storage: Option<Arc<Storage>>,
}

#[derive(Debug, Default, Error, Diagnostic)]
#[error("missing required capability")]
pub struct Incapable;

#[derive(Debug, Deref, DerefMut)]
pub struct WriteCapable<'context>(&'context mut Context);

impl<'context> WriteCapable<'context> {
    pub fn write_blob(&mut self, blob: Blob) -> Result<BlobId, impl Error> {
        todo!()
    }

    pub fn insert_response(
        &mut self,
        request: Request,
        response: Response,
    ) -> Result<(), impl Error> {
        todo!()
    }
}

#[derive(Debug, Deref, DerefMut)]
pub struct WebCapable<'context>(&'context mut Context);

impl<'context> WebCapable<'context> {
    pub fn get(&mut self, url: String) -> Result<never, never> {
        todo!()
    }
}

#[derive(Debug, Deref, DerefMut)]
pub struct TextToSpeechCapable<'context>(&'context mut Context);

impl<'context> TextToSpeechCapable<'context> {
    pub fn get(&mut self, url: String) -> Result<never, never> {
        todo!()
    }
}

impl Context {
    pub fn new(storage: impl Into<Option<Arc<Storage>>>) -> Context {
        let storage = storage.into();
        Context { storage }
    }

    pub fn write(&mut self) -> Result<WriteCapable, Incapable> {
        if true {
            Ok(WriteCapable(self))
        } else {
            Err(Incapable)
        }
    }

    pub fn get_blob(&self, id: BlobId) -> Option<Blob> {
        None
    }

    fn insert_blob(&self, data: impl Into<Blob>) {}

    pub fn get_responses(&self, request: Request) -> impl Iterator<Item = Response> {
        None
    }

    fn insert_response(&self, request: Request, response: Response) {}
}

// wait these should just be a blobid map you idiot

#[derive(Debug, Clone)]
pub struct ResponseRecord {
    pub response: Response,
    pub timestamp_inserted: u32,
    pub timestamp_revalidated: u32,
}
