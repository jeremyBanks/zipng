#![allow(unused_labels)]
#![deny(unsafe_code)]
#![warn(unused_crate_dependencies)]
#![cfg_attr(
    all(debug_assertions, any(not(test), feature = "EDITOR")),
    allow(dead_code, unreachable_code, unused_variables)
)]

use std::env;
use std::format as f;
use std::process::Termination;
use std::sync::Arc;

use tracing::warn;
use tracing_error::ErrorLayer;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::EnvFilter;

mod blob;
mod ffmpeg;
mod generic;
// mod queries;
// mod throttle;
// mod tts;
mod context;
mod engine;
mod storage;
mod query;
mod serde;

use thiserror::Error;

pub use crate::blob::Blob;
pub use crate::blob::BlobId;
pub use crate::engine::Engine;
pub use crate::generic::default;
pub use crate::generic::never;
pub use crate::generic::panic;
pub use crate::query::AnyRequest;
pub use crate::query::AnyResponse;
pub use crate::query::Context;
pub use crate::query::Request;
pub use crate::query::Response;
pub use crate::storage::sqlite::SqliteStorage;
pub use crate::storage::Storage;
pub use crate::storage::StorageError;

pub fn main() -> Result<(), panic> {
    if cfg!(debug_assertions) {
        if env::var("RUST_LOG").is_err() {
            env::set_var("RUST_LOG", f!("warn,{}=trace", env!("CARGO_CRATE_NAME")));
        }
    } else if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", f!("error,{}=warn", env!("CARGO_CRATE_NAME")));
    }

    tracing::subscriber::set_global_default(
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .pretty()
            .with_span_events(FmtSpan::CLOSE)
            .finish()
            .with(ErrorLayer::default()),
    )?;

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    // let storage: Arc<SqliteStorage> = default();
    // let engine = Engine::new(storage);

    // let request = text_to_speech("hello, world!");

    // return runtime.block_on(exercise(engine));

    std::process::exit(0)
}

// async fn exercise(engine: Engine<impl Storage>) -> Result<(), panic> {
//     let speech = engine.text_to_speech("hello, world!").await?;

//     eprintln!("{speech:?}");

//     Ok(())
// }
