use crate::never;
use serde::Serialize;
use serde::Deserialize;
use crate::context::Context;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Request {
    _reserved: never,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Response {
    _reserved: never,
}

pub fn query(request: &Request, context: &mut Context) -> Result<Response, never> {
    unreachable!()
}
