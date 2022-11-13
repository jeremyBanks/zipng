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
// pub mod throttle;
// pub mod tts;
/// Supporting types for [`Context`], and [`Metadata`]
pub mod context;
mod copyvec;
/// Supporting types for [`Engine`].
#[doc(hidden)]
pub mod engine;
/// Supporting types for [`Request`], and [`Response`].
pub mod query;
/// Supporting types for [`Storage`]
pub mod storage;

use std::ops::Deref;
use std::sync::Arc;

use query::TextToSpeech;

#[doc(inline)]
pub use crate::blobs::Blip;
#[doc(inline)]
pub use crate::blobs::Blob;
#[doc(inline)]
pub use crate::blobs::Blobbable;
#[doc(inline)]
pub use crate::context::Context;
#[doc(inline)]
pub use crate::context::Metadata;
#[doc(inline)]
pub use crate::engine::*;
#[doc(inline)]
pub use crate::generic::*;
#[doc(inline)]
pub use crate::query::AnyRequest;
#[doc(inline)]
pub use crate::query::AnyResponse;
#[doc(inline)]
pub use crate::query::Request;
#[doc(inline)]
pub use crate::query::Response;
#[doc(inline)]
pub use crate::storage::LayeredStorage;
#[doc(inline)]
pub use crate::storage::NoStorage;
#[doc(inline)]
pub use crate::storage::SqliteStorage;
#[doc(inline)]
pub use crate::storage::Storage;
#[doc(inline)]
pub use crate::storage::WebStorage;

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

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    // let request = text_to_speech("hello, world!");

    runtime.block_on(async {
        let engine = PERSISTENT.deref();
        let speech = engine.text_to_speech("hello, world!").await?;

        Result::<(), panic>::Ok(())
    })
}

// async fn exercise(engine: Engine<impl Storage>) -> Result<(), panic> {
//     let speech = engine.text_to_speech("hello, world!").await?;

//     eprintln!("{speech:?}");

//     Ok(())
// }
