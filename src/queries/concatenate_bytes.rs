use serde::Deserialize;
use serde::Serialize;

use crate::blob::Blip;
use crate::context::Context;
use crate::generic::never;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Request {
    blips: Vec<Blip>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Response {
    blip: Blip,
}

pub fn query(request: &Request, context: &mut Context) -> Result<Response, never> {
    let mut bytes: Vec<u8> = Vec::new();
    for blip in request.blips.iter() {
        let blob = context.get_blob(blip).unwrap().unwrap();
        bytes.extend(blob.as_ref());
    }
    let blip = context.insert_blob(bytes)?;
    Ok(Response { blip })
}
