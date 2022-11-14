//! it's not real
//!
//! it's [`fiction`][self]
#![warn(unused_crate_dependencies, missing_docs)]
#![allow(unused_labels, missing_docs, unused_imports)]
#![cfg_attr(
    all(debug_assertions, any(not(test), feature = "EDITOR")),
    allow(dead_code, unreachable_code, unused_variables)
)]
#![deny(unsafe_code)]

use std::env;
use std::format as f;

use tracing::warn;
use tracing_error::ErrorLayer;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::EnvFilter;

/// Supporting types for [`Blobs`][Blob] and [`Blips`][Blip].
pub mod blobs;
mod ffmpeg;
/// Generic supporting types.
#[doc(hidden)]
pub mod generic;
// pub mod queries;
pub mod throttle;
// pub mod tts;
/// Supporting types for [`Context`], and [`Metadata`]
pub mod context;
mod copyvec;
/// Supporting types for [`Engine`].
#[doc(hidden)]
pub mod engine;
mod execute;
/// Supporting types for [`Request`], and [`Response`].
pub mod query;
/// Supporting types for [`Storage`]
pub mod storage;

use std::ops::Deref;
use std::sync::Arc;

use blobs::blip::blip;
use query::TextToSpeech;
use tracing::info;

pub use crate::blobs::Blip;
pub use crate::blobs::Blob;
pub use crate::blobs::Blobbable;
pub use crate::context::Context;
pub use crate::context::Metadata;
pub use crate::engine::*;
pub use crate::generic::*;
pub use crate::query::AnyRequest;
pub use crate::query::AnyResponse;
pub use crate::query::Request;
pub use crate::query::Response;
pub use crate::storage::LayeredStorage;
pub use crate::storage::NoStorage;
pub use crate::storage::SqliteStorage;
pub use crate::storage::Storage;

/// `fiction` CLI entry point
///
/// # Panics
///
/// Panics for a variety of possible unhandled errors.
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

    color_eyre::install()?;

    info!("Initializing tokio runtime...");

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    info!("Starting main task...");

    runtime.block_on(async {
        let args: Vec<String> = std::env::args().skip(1).collect();

        info!("I'm in ur main task...");

        let engine = PERSISTENT.deref();

        info!("With ur engine...");

        let request = TextToSpeech {
            text: blip("Hello, world!"),
            ..default()
        };

        info!("Executing request... {request:?}");

        let response = engine.execute(request).await?;

        dbg!(&response);

        Ok(())
    })
}
