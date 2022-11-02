use serde::Deserialize;
use serde::Serialize;

mod concatenate_bytes;
mod concatenate_media;
mod http_get;
mod noop;
mod royalroad_chapter;
mod royalroad_fiction;
mod royalroad_spine;
mod text_to_speech;
mod text_to_speech_voices;

use derive_more::From;
use derive_more::TryInto;
use paste::paste;
use tracing::instrument;

use crate::context::Context;
use crate::generic::never;

macro_rules! request_and_response {
    ($(
        $name:ident = $tag:literal;
    )*) => { paste! {

#[derive(Debug, Serialize, Deserialize, Clone, TryInto, From)]
#[repr(u32)]
pub enum Request {
    $(
        [<$name:camel>]([<$name:snake>]::Request) = $tag,
    )*
}

#[derive(Debug, Serialize, Deserialize, Clone, TryInto, From)]
#[repr(u32)]
pub enum Response {
    $(
        [<$name:camel>]([<$name:snake>]::Response) = $tag,
    )*
}

impl traits::Request for Request {
    type Response = Response;

    #[instrument]
    fn query(&self, context: &mut Context) -> Result<Self::Response, never> {
        Ok(match self {
            $(
                Request::[<$name:camel>](request) => Response::[<$name:camel>](request.query(context)?),
            )*
        })
    }
}

impl traits::Response for Response {
    type Request = Request;
}

$(
    impl traits::Request for [<$name:snake>]::Request {
        type Response = [<$name:snake>]::Response;

        #[instrument]
        fn query(&self, context: &mut Context) -> Result<Self::Response, never> {
            [<$name:snake>]::query(self, context)
        }
    }

    impl traits::Response for [<$name:snake>]::Response {
        type Request = [<$name:snake>]::Request;
    }
)*

} }
}

request_and_response! {
    noop = 0o00;
    concatenate_bytes = 0o04;
    http_get = 0o10;
    concatenate_media = 0o14;
    text_to_speech = 0o20;
    text_to_speech_voices = 0o24;
    royalroad_fiction = 0o30;
    royalroad_spine = 0o34;
    royalroad_chapter = 0o40;
}

impl Default for Request {
    fn default() -> Self {
        Self::Noop(Default::default())
    }
}

impl Default for Response {
    fn default() -> Self {
        Self::Noop(Default::default())
    }
}

use std::fmt::Debug;

use serde::de::DeserializeOwned;
pub mod traits {
    use super::*;

    pub trait Request:
        Debug + Default + Serialize + DeserializeOwned + 'static + Into<super::Request>
    {
        type Response: Response<Request = Self>;

        fn query(&self, context: &mut Context) -> Result<Self::Response, never>;
    }

    pub trait Response:
        Debug + Default + Serialize + DeserializeOwned + 'static + Into<super::Response>
    {
        type Request: Request<Response = Self>;
    }
}
