use serde::Deserialize;
use serde::Serialize;

use derive_more::From;
use derive_more::TryInto;
use paste::paste;
use tracing::instrument;
use std::fmt::Debug;
use serde::de::DeserializeOwned;

use crate::context::Context;
use crate::generic::never;

macro_rules! request_and_response {
    () => {
        request_and_response! {
            blob = 0;
            key_value = 1;
            concatenate_bytes = 21;
            http_get = 22;
            concatenate_media = 23;
            text_to_speech = 24;
            text_to_speech_voices = 25;
            royalroad_fiction = 26;
            royalroad_spine = 27;
            royalroad_chapter = 28;
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
            $name::query(self, context)
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
        Self::Noop(Default::default())
    }
}

impl Default for Response {
    fn default() -> Self {
        Self::Noop(Default::default())
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
