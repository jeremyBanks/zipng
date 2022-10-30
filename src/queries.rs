use std::sync::Arc;

use ambassador::delegatable_trait;
use ambassador::Delegate;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;

use crate::blob::BlobId;

mod blob;
mod http;
pub mod traits;

use self::blob::BlobRequest;
use self::blob::BlobResponse;
use self::http::HttpGetRequest;
use self::http::HttpGetResponse;
use crate::blob::Blob;

#[derive(Debug, Serialize, Deserialize)]
pub struct Context {}

impl Context {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_blob(&self, id: BlobId) -> Option<Blob> {
        None
    }

    pub fn insert_blob(&mut self, data: impl Into<Blob>) -> BlobId {
        data.into().id()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[repr(u16)]
pub enum Request {
    Blob(BlobRequest) = 0xB,
    HttpGet(HttpGetRequest) = 0xF,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[repr(u16)]
pub enum Response {
    Blob(BlobResponse) = 0xB,
    HttpGet(HttpGetResponse) = 0xF,
}

impl traits::Request for Request {
    type Response = Response;

    fn query(&self, context: &mut Context) -> Self::Response {
        match self {
            Request::Blob(request) => Response::Blob(request.query(context)),
            Request::HttpGet(request) => Response::HttpGet(request.query(context)),
        }
    }
}

impl traits::Response for Response {
    fn max_age_seconds(&self) -> u32 {
        match self {
            Response::Blob(response) => response.max_age_seconds(),
            Response::HttpGet(response) => response.max_age_seconds(),
        }
    }
}
