use serde::Deserialize;
use serde::Serialize;

use crate::context::Context;
use crate::storage::StorageError;
use crate::blob::Blip;
use crate::generic::never;
use crate::generic::panic;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Request {
    key_path: Vec<Blip>
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Response {
    value: Blip
}

pub fn query(request: &Request, context: &mut Context) -> Result<Response, panic> {
    todo!()
}
