use async_trait::async_trait;

use crate::Request;

/// Common trait for types that can "execute" a [`Request`].
///
/// For users with will typically be the [`Engine`], but for implementors this
/// is often a [`Context`].
///
/// This is also implemented for [`Storage`], but that's read-only.
#[async_trait]
pub trait Execute {
    async fn execute<Request: crate::Request>(
        &self,
        request: &Request,
    ) -> Result<Request::Response, Request::Error>;
}
