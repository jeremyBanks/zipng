use serde::Deserialize;
use serde::Serialize;

use crate::context::Context;
use crate::generic::never;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Request {}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Response {}

pub fn query(request: &Request, context: &mut Context) -> Result<Response, never> {
    todo!()
}
