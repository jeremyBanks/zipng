//! don't call it The Registry
use std::fmt::Debug;

use async_trait::async_trait;
use derive_more::From;
use derive_more::TryInto;
use miette::Diagnostic;
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;

use super::RequestError;
use crate::blobs::bytes;
use crate::blobs::Postcard;
use crate::blobs::PostcardBlob;
use crate::never;
use crate::panic;
use crate::storage::Storage;
use crate::Blip;
use crate::Blob;
use crate::Blobbable;
use crate::Context;
use crate::Engine;
use crate::Request;
use crate::Response;
#[cfg(doc)]
use crate::*;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct KeyValue {
    pub key_path: Vec<Blob<bytes>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct KeyValueResponse {
    pub value: Blob<bytes>,
}

#[async_trait]
impl Request for KeyValue {
    const TAG: u32 = '/' as _;
    type Response = KeyValueResponse;
}

impl Response for KeyValueResponse {
    type Request = KeyValue;
}
