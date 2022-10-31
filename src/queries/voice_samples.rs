use serde::Deserialize;
use serde::Serialize;

use super::traits::Request;
use super::traits::Response;
use super::Context;
use crate::blob::Blob;
use std::collections::BTreeMap;
use crate::blob::BlobId;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct VoiceSamplesRequest;

impl super::Request {
    pub fn voice_samples() -> Self {
        Self::VoiceSamples(VoiceSamplesRequest)
    }
}

impl Request for VoiceSamplesRequest {
    type Response = VoiceSamplesResponse;

    fn query(&self, context: &mut Context) -> Self::Response {
        // get all known TTS voices, then for each, generate a sample
        // utterance and store it in the context blob store.

        todo!()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct VoiceSamplesResponse {
    samples: BTreeMap<String, VoiceSamplesUtterance>
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct VoiceSamplesUtterance {
    text: String,
    blob: BlobId,
}

impl Response for VoiceSamplesResponse {}
