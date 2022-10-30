use std::sync::Arc;

use ambassador::delegatable_trait;
use ambassador::Delegate;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;

use crate::blob::BlobId;

pub mod traits {
    use super::*;

    pub trait Request: Serialize + DeserializeOwned {
        type Response: traits::Response;

        fn query(&self, context: &mut Context) -> Self::Response;
    }

    pub trait Response: Serialize + DeserializeOwned {
        const NO_SAVE: u32 = 0x_______0; // no cache
        const BRIEFLY: u32 = 0x______10; // 16 seconds
        const FOR_NOW: u32 = 0x____1000; // ~2 hours
        const A_WHILE: u32 = 0x__300000; // ~1 month
        const FOREVER: u32 = 0x10000000; // ~8 years

        fn max_age_seconds(&self) -> u32 {
            Self::FOR_NOW
        }
    }
}
use traits as t;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[repr(u16)]
pub enum Request {
    Blob(BlobRequest) = 0xB,
    Fetch(FetchRequest) = 0xF,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[repr(u16)]
pub enum Response {
    Blob(BlobResponse) = 0xB,
    Fetch(FetchResponse) = 0xF,
}

impl t::Request for Request {
    type Response = Response;

    fn query(&self, context: &mut Context) -> Self::Response {
        match self {
            Request::Blob(request) => Response::Blob(request.query(context)),
            Request::Fetch(request) => Response::Fetch(request.query(context)),
        }
    }
}

impl t::Response for Response {
    fn max_age_seconds(&self) -> u32 {
        match self {
            Response::Blob(response) => response.max_age_seconds(),
            Response::Fetch(response) => response.max_age_seconds(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct BlobRequest {
    id: BlobId,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct BlobResponse {
    data: Option<Arc<[u8]>>,
}

impl t::Response for BlobResponse {
    fn max_age_seconds(&self) -> u32 {
        Self::NO_SAVE
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FetchRequest {
    url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FetchResponse {
    status: u16,
    body: Arc<[u8]>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Context {}

impl Context {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_blob(&self, id: BlobId) -> Vec<u8> {
        todo!()
    }

    pub fn insert_blob(&mut self, data: Vec<u8>) -> BlobId {
        todo!()
    }
}
