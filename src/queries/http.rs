use serde::Deserialize;
use serde::Serialize;

use super::traits::Request;
use super::traits::Response;
use super::Context;
use crate::blob::Blob;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct HttpGetRequest {
    url: String,
}

impl Request for HttpGetRequest {
    type Response = HttpGetResponse;

    fn query(&self, context: &mut Context) -> Self::Response {
        todo!()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct HttpGetResponse {
    status: u16,
    body: Blob,
}

impl Response for HttpGetResponse {
    fn max_age_seconds(&self) -> u32 {
        match self.status {
            200 | 301 | 308 => Self::A_WHILE,
            302 | 303 | 307 | 400 | 405 => Self::FOR_NOW,
            406..=599 => Self::BRIEFLY,
            _ => Self::NO_SAVE,
        }
    }
}
