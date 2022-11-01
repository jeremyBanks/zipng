use serde::Deserialize;
use serde::Serialize;

use crate::context::Context;
use crate::generic::never;

impl super::Request {
    pub fn http_get(url: impl AsRef<str>) -> Result<Self, never> {
        let url = url.as_ref().to_string();
        Ok(Self::from(Request { url }))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Request {
    url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Response {}

pub fn query(request: &Request, context: &mut Context) -> Result<Response, never> {
    todo!()
}
