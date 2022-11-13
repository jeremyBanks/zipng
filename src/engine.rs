use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::RwLock;

use tokio::runtime::Handle;

use crate::context::Context;
use crate::default;
use crate::never;
use crate::panic;
use crate::Blob;
use crate::Storage;

/// `Engine` is the main entry point for the library, connecting the storage
/// backend with the query engine.
#[derive(Debug)]
pub struct Engine {
    storage: Arc<dyn Storage>,
    runtime: tokio::runtime::Handle,
}

impl Engine {
    /// Creates a new `Engine` with the given storage backend.
    pub fn new(storage: Arc<dyn Storage>) -> Engine {
        Self {
            storage,
            runtime: Handle::current(),
        }
    }

    /// Executes a query, returning either a new `Response` or a cached one from
    /// the backing storage.
    pub async fn execute<Request: crate::Request>(
        &self,
        request: Request,
    ) -> Result<Request::Response, never> {
        let request_blip = request.to_blob().blip();

        // let context = Context::new(&request, &self.storage);

        // let response = request.execute(&mut context).await?;

        // self.storage.insert_response(&request, &response).await?;

        todo!()
    }
}
