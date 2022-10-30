use std::fmt::Debug;

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::queries::Context;

pub trait Request: Debug + Serialize + DeserializeOwned + 'static + Into<super::Request> {
    type Response: Response;

    fn query(&self, context: &mut Context) -> Self::Response;
}

pub trait Response: Debug + Serialize + DeserializeOwned + 'static + Into<super::Response> {}
