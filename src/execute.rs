use async_trait::async_trait;

use crate::blobs::blip;
use crate::blobs::blip::blip;
use crate::blobs::blob;
use crate::blobs::Blip;
use crate::blobs::Blob;
use crate::blobs::BlobSerialization;
use crate::query;
use crate::query::RequestError;
use crate::storage::StorageError;
use crate::Blobbable;
use crate::Request;

#[async_trait]
pub trait Incremental {
    async fn blip<T: Blobbable + ?Sized, S: BlobSerialization>(
        blob: blob::Blob<T, S>,
    ) -> Result<blip::Blip<T, S>, StorageError> {
        // let blip = blob.blip();
        // if !blip.is_inline() {
        //     // just remove the fucking query alltogether you idiot
        //     // it's just a distraction
        //     self.set(&blip, &blob);
        // }
        // Ok(blip)
        todo!()
    }

    async fn get<Request: crate::Request>(
        &self,
        request: &Request,
    ) -> Result<Request::Response, RequestError> {
        self.get(self.get_blip(blip(request)))
    }

    async fn set<Request: crate::Request>(
        &self,
        request: &Request,
        response: &Request::Response,
    ) -> Result<(), StorageError>;

    async fn get_blip<Request: crate::Request>(
        &self,
        request: Blip<Request>,
    ) -> Result<Blip<Request::Response>, RequestError>;

    async fn set_blip<Request: crate::Request>(
        &self,
        request: Blip<Request>,
        response: Blip<Request>,
    ) -> Result<(), RequestError>;
}
