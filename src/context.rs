use std::sync::Arc;

use miette::Diagnostic;
use thiserror::Error;

use crate::blob::Blob;
use crate::blob::BlobId;
use crate::generic::never;
use crate::generic::panic;
use crate::queries::Request;
use crate::queries::Response;
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

    pub fn query(&mut self, request: Request) -> Result<Response, panic> {
        todo!()
    }

    pub fn get_blob(&self, id: impl Into<BlobId>) -> Option<Blob> {
        todo!()
    }

    pub fn insert_blob(&self, data: impl Into<Blob>) -> never {
        todo!()
    }

    pub fn get_responses(&self, request: Request) -> never {
        todo!()
    }

    pub fn insert_response(&self, request: Request, response: Response) {}
    fn insert_response_blob(&self, request: impl Into<BlobId>, response: impl Into<BlobId>) {}
}

#[derive(Debug, Clone)]
pub struct ResponseRecord {
    pub response: Response,
    /// the first time a request-response pair is inserted, this is set
    pub timestamp_inserted: u32,
    /// re-inserting the same request-response pair will update this
    pub timestamp_revalidated: u32,
}

pub trait Capabilities {
    fn can_write(&self) -> bool {
        true
    }
    fn can_use_text_to_speech(&self) -> bool {
        true
    }
    fn can_use_internet(&self) -> bool {
        true
    }
}

impl Capabilities for &mut Context {}
