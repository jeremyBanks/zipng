use std::collections::BTreeMap;
use std::time::Duration;

use serde::Deserialize;
use serde::Serialize;

use crate::blob::Blob;
use crate::blob::BlobId;
use crate::context::Context;
use crate::generic::never;
use crate::generic::panic;

impl super::Request {
    pub fn http_get(url: impl AsRef<str>) -> Result<Self, never> {
        let url = url.as_ref().to_string();
        Ok(Self::from(Request { url }))
    }
}

impl crate::context::Context {
    pub fn http_get(&mut self, url: impl AsRef<str>) -> Result<Blob, never> {
        let request = Request { url: url.as_ref().to_string() };
        todo!()
        // let response = self.query(Request { url: url.as_ref().to_string() })?;
        // let blob_id = response.body;
        // let blob = self.get_blob(&blob_id).unwrap().unwrap();
        // Ok(blob)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Request {
    url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Response {
    status: u16,
    body: BlobId,
    headers: BTreeMap<BlobId, Vec<BlobId>>,
}

pub fn query(request: &Request, context: &mut Context) -> Result<Response, never> {
    Ok((|| -> Result <Response, panic> {
        std::thread::sleep(Duration::from_secs(1));

        let url = &request.url;
        let response = reqwest::blocking::get(url)?;

        let status = response.status().as_u16();

        let body = todo!(); //context.insert_blob(response.bytes()?.as_ref())?;

        let headers: BTreeMap<BlobId, Vec<BlobId>> = BTreeMap::new();
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
