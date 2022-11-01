use serde::Deserialize;
use serde::Serialize;

use crate::context::Context;
use crate::generic::never;

impl super::Request {
    pub fn text_to_speech_voices() -> Result<Self, never> {
        Ok(Self::from(Request {}))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Request {}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Response {}

pub fn query(request: &Request, context: &mut Context) -> Result<Response, never> {
    todo!()
}
