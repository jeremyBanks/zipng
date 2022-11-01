use serde::Deserialize;
use serde::Serialize;

use crate::blob::BlobId;

pub mod traits;

use tracing::instrument;

use crate::context::Context;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[repr(u8)]
pub enum Request {
    /// Converts plain text to speech as a WebM audio file.
    TextToSpeech {
        text: BlobId,
        voice_name: Option<BlobId>,
        voice_language: Option<BlobId>,
    } = 21,

    /// Returns a list of the currently supported text-to-speech voices
    /// on this system.
    TextToSpeechVoices = 22,

    /// Concatenate multiple audio/video files into a single WebM/MKV file.
    ConcatenateMedia { media_files: Vec<BlobId> } = 23,

    /// An HTTP GET request to a URL.
    HttpGet { url: BlobId } = 24,

    /// Returns the internet-archived HTTP response for the given URL,
    /// as close to the indicated timestamp as possible.
    ArchiveGet { url: BlobId, timestamp: Option<u64> } = 25,

    /// Returns a list of archive timestamps for a given URL.
    ArchiveGetTimestamps { url: BlobId } = 26,

    /// Returns a full audio book as a WebM audio file with chapters
    /// markers and embedded text tracks, for a fiction on RoyalRoad.
    RoyalRoadAudioBook { fiction_id: u64 } = 27,

    /// Returns a full ePub reflowing audio ebook for a fiction on RoyalRoad.
    /// The outer ePub zip file will not use compression.
    RoyalRoadAudioEbook {
        fiction_id: u64,
        voice_name: Option<BlobId>,
        voice_language: Option<BlobId>,
    } = 28,

    /// The full contents of a fiction from RoyalRoad.
    RoyalRoadFiction { fiction_id: u64 } = 29,

    /// The "spine" metadata for a fiction from RoyalRoad.
    RoyalRoadFictionSpine { fiction_id: u64 } = 30,

    /// The contents of a chapter from a fiction on RoyalRoad.
    RoyalRoadFictionChapter { fiction_id: u64, chapter_id: u64 } = 31,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[repr(u8)]
pub enum Response {
    TextToSpeech = 21,

    TextToSpeechVoices = 22,

    ConcatenateMedia = 23,

    HttpGet = 24,

    ArchiveGet = 25,

    ArchiveGetTimestamps = 26,

    RoyalRoadAudioBook = 27,

    RoyalRoadAudioEbook = 28,

    RoyalRoadFiction = 29,

    RoyalRoadFictionSpine = 30,

    RoyalRoadFictionChapter = 31,
}

impl traits::Request for Request {
    type Response = Response;

    #[instrument]
    fn query(&self, context: &mut Context) -> Self::Response {
        match self {
            _ => todo!()
            // Request::HttpGet(request) => Response::HttpGet(request.query(context)),
            // Request::TextToSpeech(request) => Response::TextToSpeech(request.query(context)),
        }
    }
}

impl traits::Response for Response {}
