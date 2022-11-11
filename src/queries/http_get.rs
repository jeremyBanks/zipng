use std::collections::BTreeMap;
use std::time::Duration;

use serde::Deserialize;
use serde::Serialize;

use crate::blob::Blip;
use crate::context::Context;
use crate::generic::never;
use crate::generic::panic;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Request {
    url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Response {
    status: u16,
    body: Blip,
    headers: BTreeMap<Blip, Vec<Blip>>,
}

pub fn query(request: &Request, context: &mut Context) -> Result<Response, never> {
    Ok((|| -> Result <Response, panic> {
        std::thread::sleep(Duration::from_secs(1));

        let url = &request.url;
        let response = reqwest::blocking::get(url)?;

        let status = response.status().as_u16();

        let body = todo!(); //context.insert_blob(response.bytes()?.as_ref())?;

        let headers: BTreeMap<Blip, Vec<Blip>> = BTreeMap::new();
        for (name, value) in response.headers().iter() {
            // let name = context.insert_blob(name.as_str().as_bytes())?;
            // let value = context.insert_blob(value.as_bytes())?;
            // headers.entry(name).or_default().push(value);
        }

        Ok(Response {
            status,
            headers,
            body,
        })
    })()
    .unwrap_or_default())
}
