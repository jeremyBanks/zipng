#![cfg_attr(debug_assertions, allow(unused))]
use std::collections::HashMap;
use std::convert::Infallible;
use std::env;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::format as f;
use std::hash::Hasher;
use std::str;

use arrayvec::ArrayVec;
use bstr::BStr;
use bstr::BString;
use derive_more::AsMut;
use derive_more::AsRef;
use derive_more::Deref;
use derive_more::From;
use derive_more::Into;
use digest::generic_array::GenericArray;
use digest::Digest;
use rusqlite::blob::Blob;
use rusqlite::functions::FunctionFlags;
use rusqlite::LoadExtensionGuard;
use rusqlite::OptionalExtension;
use rusqlite_migration::Migrations;
use rusqlite_migration::M;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use tracing::debug;
use tracing::error;
use tracing::info;
use tracing::instrument;
use tracing::metadata::LevelFilter;
use tracing::trace;
use tracing::warn;
use tracing_error::ErrorLayer;
use tracing_error::SpanTrace;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::EnvFilter;
use twox_hash::Xxh3Hash64;
use typenum::U20;

#[derive(Debug, From, AsRef, AsMut)]
pub struct Connection {
    connection: rusqlite::Connection,
}

const APPLICATION_ID: u32 = 0x_F_1C_15_00;

impl Connection {
    pub fn new(mut connection: rusqlite::Connection) -> Result<Self, rusqlite::Error> {
        info!("Initializing connection...");

        info!("Loading sqlite_zstd extension...");
        unsafe {
            let guard = LoadExtensionGuard::new(&connection)?;
            connection.load_extension("sqlite_zstd", None)?;
        };

        let (application_id, user_version, schema_version) = connection.query_row(
            "select
                application_id ,
                user_version ,
                schema_version
            from
                pragma_application_id join
                pragma_user_version join
                pragma_schema_version",
            (),
            |row| {
                Ok((
                    row.get_ref(0)?.as_i64()? as u32,
                    row.get_ref(1)?.as_i64()? as u32,
                    row.get_ref(2)?.as_i64()? as u32,
                ))
            },
        )?;

        trace!(
            application_id = application_id,
            user_version = user_version,
            schema_version = schema_version
        );

        if application_id == 0 || application_id != APPLICATION_ID {
            if application_id == 0 {
                info!("initializing application_id to {APPLICATION_ID:08X}");
                connection.pragma_update(None, "application_id", APPLICATION_ID)?;
            }

            connection.pragma_update(None, "page_size", 64 * 1024)?;
            connection.pragma_update(None, "cache_size", -1 * 64 * 1024 * 1024)?;
            connection.pragma_update(None, "auto_vacuum", "full")?;
            connection.pragma_update(None, "foreign_keys", true)?;
            connection.pragma_update(None, "synchronous", "normal")?;
            connection.pragma_update(None, "temp_store", "memory")?;
            connection.pragma_update(None, "secure_delete", true)?;
            connection.pragma_update(None, "journal_mode", "wal")?;

            Migrations::new(vec![
                M::up(
                    r#"
                    create table BlobStore(
                        blake3 Blob primary key,
                        bytes Blob not null
                    ) strict;
                "#,
                ),
                M::up(
                    r#"
                    commit; -- rusqlite_migrations compatibility hack
                    select zstd_enable_transparent( '{
                        "table": "BlobStore",
                        "column": "bytes",
                        "compression_level": 21,
                        "dict_chooser": "''BlobStore''"
                    }');
                    begin transaction; -- rusqlite_migrations compatibility hack
                "#,
                ),
                M::up(
                    r#"
                    create table if not exists QueryCache(
                        request_blob_id Blob not null,
                        response_blob_id Blob not null,
                        timestamp Integer not null default( CURRENT_TIMESTAMP ),
                        status Blob default( null ) check( status is null or length(status) <= 8 ),
                        foreign key( request_blob_id ) references BlobStore( blob_id ),
                        foreign key( response_blob_id ) references BlobStore( blob_id ),
                        unique( request_blob_id, timestamp, status, response_blob_id )
                    ) strict;
                "#,
                ),
            ])
            .to_latest(&mut connection)
            .unwrap();
        } else {
            error!(
                "database has an unexpected application_id: {application_id:08X}. skipping \
                 migrations and pragmas."
            );
        }

        info!("optimizing database");
        connection.execute_batch(
            r#"
                select zstd_incremental_maintenance(null, 0.9375);
                vacuum;
                analyze;
            "#,
        )?;

        {
            let mut q = connection
                .prepare("select max(length) from BlobStore")
                .unwrap();
            let result = q.query_row((), |row| Ok(format!("{:?}", row.get_ref(0))));
            dbg!(result);
        }

        Ok(Self { connection })
    }

    pub fn open_in_memory() -> Result<Self, rusqlite::Error> {
        Self::new(rusqlite::Connection::open_in_memory()?)
    }

    pub fn open(path: &str) -> Result<Self, rusqlite::Error> {
        Self::new(rusqlite::Connection::open(path)?)
    }
}

fn main() -> Result<(), eyre::Report> {
    if cfg!(debug_assertions) {
        if env::var("RUST_LOG").is_err() {
            env::set_var("RUST_LOG", f!("warn,{}=trace", env!("CARGO_CRATE_NAME")));
        }
    } else {
        if env::var("RUST_LOG").is_err() {
            env::set_var("RUST_LOG", f!("error,{}=warn", env!("CARGO_CRATE_NAME")));
        }
    }

    color_eyre::install()?;

    tracing::subscriber::set_global_default(
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .pretty()
            .with_span_events(FmtSpan::CLOSE)
            .finish()
            .with(ErrorLayer::default()),
    )?;

    let mut connection = Connection::open("data/test.sqlite")?;

    Ok(())
}
