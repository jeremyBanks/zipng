use std::fmt::Debug;

use derive_more::From;
use derive_more::TryInto;
use paste::paste;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;
use tracing::instrument;

use crate::context::Context;
use crate::generic::default;
use crate::generic::never;

macro_rules! request_and_response {
    () => {
        request_and_response! {
            blob_from_id     = 0x00; // raw blob storage
            blob_from_sha1   = 0x01;
            blob_from_sha256 = 0x02;
            blob_from_blake3 = 0x03; // (length = 32)
            blob_from_gitsha = 0x04; // (bytes, object type = "blob")
            blob_from_key    = 0x08; // arbitrary keypath-value storage
            blobs_by_keys_zipped = 0x0B;
            mkv_by_concatenation = 0x0C;

            http_get = 0x09;

            http_archive_get = 0x1A;
            http_archive_query = 0x1B;

            royalroad_chapter = 0x48;
            royalroad_fiction = 0x46;
            royalroad_spine = 0x47;

            speech_voices    = 0x30;
            speech_from_text = 0x31;
            speech_from_ssml = 0x32;
            speech_from_html = 0x33;

            exercised_voices = 0x36;
        }
    };
    ( $( $name:ident = $tag:literal; )* ) => {
        paste! {
            request_and_response! {
                $( [<$name:snake>]: [<$name:camel>] = $tag; )*
            }
        }
    };
    ($(
        $name:ident: $Name:ident = $tag:literal;
    )*) => {
$(mod $name;)*

#[derive(Debug, Serialize, Deserialize, Clone, TryInto, From)]
#[repr(u32)]
pub enum Request {
    $(
        $Name($name::Request) = $tag,
    )*
}

#[derive(Debug, Serialize, Deserialize, Clone, TryInto, From)]
#[repr(u32)]
pub enum Response {
    $(
        $Name($name::Response) = $tag,
    )*
}

impl traits::Request for Request {
    type Response = Response;

    #[instrument]
    fn query(&self, context: &mut Context) -> Result<Self::Response, never> {
        Ok(match self {
            $(
                Request::$Name(request) => Response::$Name(request.query(context)?),
            )*
        })
    }
}

$(
    impl traits::Request for $name::Request {
        type Response = $name::Response;

        #[instrument]
        fn query(&self, context: &mut Context) -> Result<Self::Response, never> {
            Ok($name::query(self, context)?)
        }
    }

    impl traits::Response for $name::Response {
        type Request = $name::Request;
    }
)*

} }

impl traits::Response for Response {
    type Request = Request;
}

request_and_response! {}

impl Default for Request {
    fn default() -> Self {
        Self::Blob(default())
    }
}

impl Default for Response {
    fn default() -> Self {
        Self::Blob(default())
    }
}

pub mod traits {
    use super::*;

    pub trait Request:
        Debug + Default + Serialize + DeserializeOwned + Send + 'static + Into<super::Request>
    {
        type Response: Response<Request = Self>;

        fn query(&self, context: &mut Context) -> Result<Self::Response, never>;
    }

    pub trait Response:
        Debug + Default + Serialize + DeserializeOwned + Send + 'static + Into<super::Response>
    {
        type Request: Request<Response = Self>;
    }
}
