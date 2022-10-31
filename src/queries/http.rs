use serde::Deserialize;
use serde::Serialize;

use super::traits::Request;
use super::traits::Response;
use super::Context;
use crate::blob::Blob;

impl super::Request {
    pub fn http_get(url: String) -> Self {
        Self::HttpGet(HttpGetRequest { url })
    }
}

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

impl Response for HttpGetResponse {}
