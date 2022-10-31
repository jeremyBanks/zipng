use std::sync::Arc;

use ambassador::delegatable_trait;
use ambassador::Delegate;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;

use crate::blob::BlobId;

mod http;
pub mod traits;
mod voice_samples;

use derive_more::From;
use derive_more::TryInto;

use self::http::HttpGetRequest;
use self::http::HttpGetResponse;
use self::voice_samples::VoiceSamplesRequest;
use self::voice_samples::VoiceSamplesResponse;
use crate::blob::Blob;
use crate::context::Context;
use crate::storage::Storage;

#[derive(Debug, Serialize, Deserialize, Clone, From, TryInto)]
#[repr(u8)]
pub enum Request {
    HttpGet(HttpGetRequest) = 0x0F,
    VoiceSamples(VoiceSamplesRequest) = 0x1C,
}

#[derive(Debug, Serialize, Deserialize, Clone, From, TryInto)]
#[repr(u8)]
pub enum Response {
    HttpGet(HttpGetResponse) = 0x0F,
    VoiceSamples(VoiceSamplesResponse) = 0x1C,
}

impl traits::Request for Request {
    type Response = Response;

    fn query(&self, context: &mut Context) -> Self::Response {
        match self {
            Request::HttpGet(request) => Response::HttpGet(request.query(context)),
            Request::VoiceSamples(request) => Response::VoiceSamples(request.query(context)),
        }
    }
}

impl traits::Response for Response {}
