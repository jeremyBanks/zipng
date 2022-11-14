mod layered;
mod no;
mod sqlite;
mod web;

use std::fmt::Debug;

use miette::Diagnostic;
use static_assertions::assert_obj_safe;
use thiserror::Error;
use tracing::error;

pub use self::layered::*;
pub use self::no::*;
pub use self::sqlite::*;
pub use self::web::*;
use crate::blobs::Blip;
use crate::blobs::UnknownBlip;
use crate::blobs::UnknownBlob;
use crate::unknown;
use crate::AnyRequest;
use crate::AnyResponse;
use crate::Blob;
use crate::Blobbable;
use crate::Metadata;
use crate::Response;
#[cfg(doc)]
use crate::*;

/// TODO: this probably needs to be an async_trait

#[derive(Debug, Error, Diagnostic)]
#[error("{self:?}")]
pub enum StorageError {
    #[error("storage operation not supported")]
    Unsupported,
    #[error("storage operation not allowed")]
    Denied,
    #[error("storage operation failed")]
    Failed,
}

assert_obj_safe!(StorageImpl);

/// A storage backend implementation. This is a non-object-safe extension trait
/// over [`StorageImpl`].
impl<S> Storage for S where S: StorageImpl + ?Sized {}
pub trait Storage: StorageImpl {
    fn insert_blob<T: Blobbable + ?Sized>(&self, blob: Blob<T>) -> Result<Blip<T>, StorageError> {
        // Ok(self.insert_blob_bytes(blob.retype())?.retype())
        todo!()
    }

    fn get_blob<T: Blobbable + ?Sized>(&self, blip: Blip<T>) -> Result<Blob<T>, StorageError> {
        // Ok(self.get_blob_bytes(blip.retype())?.retype())
        todo!()
    }
}

pub struct ResponseItem<Request: crate::Request> {
    pub request: Blip<Request>,
    pub response: Blip<Request::Response>,
    pub metadata: Blip<Metadata>,
}

/// A storage backend implementation. For the sake of object safety, methods on
/// this trait not generic, and typically operate on untyped byte blobs. Users
/// should use the [`Storage`] trait instead, which provides more strongly-typed
/// generic wrappers.
pub trait StorageImpl: Debug + Send + Sync {
    fn insert_blob_impl(&self, blob: &UnknownBlob) -> Result<UnknownBlip, StorageError> {
        Err(StorageError::Unsupported)
    }

    fn get_blob_impl(&self, blip: UnknownBlip) -> Result<Option<UnknownBlob>, StorageError> {
        Err(StorageError::Unsupported)
    }

    fn insert_response_impl(
        &self,
        request: UnknownBlip,
        response: UnknownBlip,
    ) -> Result<UnknownResponseItem, StorageError> {
        Err(StorageError::Unsupported)
    }

    fn get_response_impl(
        &self,
        request: UnknownBlip,
    ) -> Result<Option<UnknownResponseItem>, StorageError> {
        Err(StorageError::Unsupported)
    }

    fn get_responses_impl(
        &self,
        request: UnknownBlip,
    ) -> Result<Box<dyn Iterator<Item = UnknownResponseItem>>, StorageError> {
        Ok(Box::new(self.get_response_impl(request)?.into_iter()))
    }
}

pub struct UnknownResponseItem {
    pub request: UnknownBlip,
    pub response: UnknownBlip,
    pub metadata: Blip<Metadata>,
}

// #[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// struct Query<Request: crate::Request> {
//     request: Request,
//     response: Request::Response,
//     created_at_ns: i64,
//     validated_at_ns: i64,
//     read: Vec<Blip<unknown>>,
//     wrote: Vec<Blip<unknown>>,
// }

// / A storage backend implementation. For the sake of object safety, methods on
// / this trait not generic, and typically operate on untyped byte blobs. Users
// / should use the [`Storage`] trait instead, which provides more
// strongly-typed / generic wrappers.
// pub trait StorageImpl: Debug + Send + Sync {
//     fn insert_blob_bytes(&self, blob: UnknownBlob) -> Result<UnknownBlip,
// StorageError> {         Err(StorageError::Unsupported)
//     }

//     fn get_blob_bytes(&self, blip: UnknownBlip) -> Result<UnknownBlob,
// StorageError> {         Err(StorageError::Unsupported)
//     }

//     fn get_responses(
//         &self,
//         request: AnyRequest,
//     ) -> Result<Box<dyn Iterator<Item = AnyResponse>>, StorageError> {
//         Err(StorageError::Unsupported)
//     }

//     // fn insert_response_id(
//     //     &self,
//     //     request_id: Blip,
//     //     response_id: Blip,
//     // ) -> Result<(), StorageError> {
//     //     Err(StorageError::Unsupported)
//     // }

//     // fn get_response_ids(
//     //     &self,
//     //     request_id: Blip,
//     // ) -> Result<Box<dyn Iterator<Item = Result<ResponseIdRecord,
// StorageError>>>,     // StorageError> {
//     //     Err(StorageError::Unsupported)
//     // }

//     // fn get_response_id(
//     //     &self,
//     //     request_id: Blip,
//     // ) -> Result<Option<ResponseIdRecord>, StorageError> {
//     //     if let Some(result) = self.get_response_ids(request_id)?.next() {
//     //         Ok(Some(result?))
//     //     } else {
//     //         Ok(None)
//     //     }
//     // }

//     // fn insert_response<Request: crate::queries::Request>(
//     //     &self,
//     //     request: &Request,
//     //     response: &Request::Response,
//     // ) -> Result<(), StorageError> {
//     //     todo!()
//     // }

//     // fn get_responses<Request: crate::queries::Request>(
//     //     &self,
//     //     request: &Request,
//     // ) -> Result<
//     //     Box<dyn Iterator<Item = Result<ResponseRecord<Request::Response>,
//     // StorageError>>>,     StorageError,
//     // > { todo!()
//     // }

//     // fn get_response<Request: crate::queries::Request>(
//     //     &self,
//     //     request: &Request,
//     // ) -> Result<Option<ResponseRecord<Request::Response>>, StorageError> {
//     //     if let Some(result) = self.get_responses(request)?.next() {
//     //         Ok(Some(result?))
//     //     } else {
//     //         Ok(None)
//     //     }
//     // }

//     // // XXX: move these to context!
//     // // storage is internal and it can be simple and dumb.
//     // fn get_responses_where<Request: crate::queries::Request>(
//     //     &self,
//     //     request: &Request,
//     //     predicate: impl 'static + Fn(&ResponseRecord<Request::Response>)
// -> bool,     // ) -> Result<
//     //     Box<dyn Iterator<Item = Result<ResponseRecord<Request::Response>,
//     // StorageError>>>,     StorageError,
//     // > { let responses = self.get_responses(request)?;
//     // > Ok(Box::new(responses.filter_map( move |response| match response {
//     // > Ok(response) => if predicate(&response) { Some(Ok(response)) } else
// { None     // > }, Err(error) => Some(Err(error)), }, )))
//     // }

//     // fn get_response_where<Request: crate::queries::Request>(
//     //     &self,
//     //     request: &Request,
//     //     predicate: impl 'static + Fn(&ResponseRecord<Request::Response>)
// -> bool,     // ) -> Result<Option<ResponseRecord<Request::Response>>,
// StorageError> {     //     if let Some(result) =
// self.get_responses_where(request,     // predicate)?.next() {
// Ok(Some(result?))     //     } else {
//     //         Ok(None)
//     //     }
//     // }
// }

// #[derive(Debug, Clone)]
// pub struct RequestRecord<Request: crate::Request> {
//     pub response_blip: Blip<Request>,
//     pub inserted_at: u32,
//     pub validated_at: u32,
// }
