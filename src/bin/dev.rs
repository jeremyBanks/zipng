#![cfg_attr(debug_assertions, allow(unused))]
use std::collections::HashMap;
use std::convert::Infallible;
use std::env;
use std::fmt::Debug;
use std::format as f;

use derive_more::AsMut;
use derive_more::AsRef;
use derive_more::Deref;
use derive_more::From;
use derive_more::Into;
use digest::generic_array::GenericArray;
use digest::Digest;
use rusqlite::blob::Blob;
use rusqlite::config::DbConfig::SQLITE_DBCONFIG_ENABLE_FKEY;
use rusqlite::functions::FunctionFlags;
use rusqlite::LoadExtensionGuard;
use rusqlite::OptionalExtension;
use serde::Deserialize;
use serde::Serialize;
use sha1;
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
use typenum::U20;

#[derive(Debug, Into, From, AsRef, AsMut)]
pub struct Connection {
    connection: rusqlite::Connection,
    zstd_enabled: bool,
}

impl Connection {
    pub fn new(connection: rusqlite::Connection) -> Self {
        add_functions(&mut connection).unwrap();
        fn add_functions(connection: &mut rusqlite::Connection) -> Result<(), rusqlite::Error> {
            connection.create_scalar_function(
                "blob_id",
                1,
                FunctionFlags::SQLITE_DETERMINISTIC,
                |ctx| Ok(blob_id(ctx.get_raw(0).as_bytes()?)),
            )?;

            connection.create_scalar_function(
                "blake3",
                1,
                FunctionFlags::SQLITE_DETERMINISTIC,
                |ctx| Ok(blake3::hash(ctx.get_raw(0).as_bytes()?).as_bytes()),
            )?;

            connection.create_scalar_function(
                "sha1",
                1,
                FunctionFlags::SQLITE_DETERMINISTIC,
                |ctx| Ok(sha1::Sha1::digest(ctx.get_raw(0).as_bytes()?).as_slice()),
            )?;
            Ok(())
        }


        add_extensions(&mut connection).unwrap();
        fn add_extensions(connection: &mut rusqlite::Connection) -> Result<(), rusqlite::Error> {
            Ok(unsafe {
                let guard = LoadExtensionGuard::new(&connection)?;
                connection.load_extension("sqlite_zstd", None)?;
            })
        }

        set_pragmas(&mut connection).unwrap();
        fn set_pragmas(connection: &mut rusqlite::Connection) -> Result<(), rusqlite::Error> {
            connection.pragma_update(None, "foreign_keys", true)?;
            connection.pragma_update(None, "auto_vacuum", "full")?;
            connection.pragma_update(None, "journal_mode", "wal")?;
            connection.pragma_update(None, "synchronous", "normal")?;
            connection.pragma_update(None, "temp_store", "memory")?;
            connection.pragma_update(None, "page_size", 64 * 1024)?;
            connection.pragma_update(None, "cache_size", 64 * 1024 * 1024)?;
            connection.pragma_update(None, "application_id", 0xF1C_15_F1C_u32)?;
            connection.pragma_update(None, "user_version", 0xF1C15_001_u32)?;
            Ok(())
        }

        let zstd_enabled = unsafe {
            let guard = LoadExtensionGuard::new(&connection);
            guard.is_ok() && connection.load_extension("sqlite_zstd", None).is_ok()
        };

        if false
            == connection
                .query_row(
                    "SELECT count(*) FROM sqlite_master WHERE type='table' AND name='BlobStore'",
                    (),
                    |row| row.get(0),
                )
                .unwrap()
        {}

        if zstd_enabled {
            connection
                .query_row(
                    "select zstd_enable_transparent( ? )",
                    &[r#"{
                "table": "BlobStore",
                "column": "bytes",
                "compression_level": 21,
                "dict_chooser": "'BlobStore'"
            }"#],
                    |_| Ok(()),
                )
                .unwrap();
        }

        Self {
            connection,
            zstd_enabled,
        }
    }

    pub fn open_in_memory() -> Self {
        Self::new(rusqlite::Connection::open_in_memory().unwrap())
    }

    pub fn open(path: &str) -> Result<Self, rusqlite::Error> {
        Ok(Self::new(rusqlite::Connection::open(path)?))
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        if self.zstd_enabled {
            self.connection
                .execute_batch("zstd_incremental_maintenance(null, 1)")
                .unwrap();
        }
        self.connection.execute_batch("analyze").unwrap();
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), eyre::Report> {
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

    println!("{:02X?}", blob_id(b""));

    Ok(())
}

pub type GitId = [u8; 20];
pub type Blake3 = [u8; 32];

fn blob_id(bytes: &[u8]) -> GitId {
    sha1::Sha1::new()
        .chain_update("blob")
        .chain_update(" ")
        .chain_update(bytes.len().to_string())
        .chain_update([0x00])
        .chain_update(&bytes)
        .finalize()
        .into()
}

trait QueryCache: BlobStore {
    type QueryCacheError;
    fn init_query_cache(&mut self) -> Result<(), Self::QueryCacheError> {
        Ok(())
    }
}

trait BlobStore {
    type BlobStoreError;
    fn init_blob_cache(&mut self) -> Result<(), Self::BlobStoreError> {
        Ok(())
    }
    fn has_blob(&self, blob_id: GitId) -> Result<bool, Self::BlobStoreError> {
        Ok(self.get_blob(blob_id)?.is_none())
    }
    fn insert_blob(&mut self, bytes: &[u8]) -> Result<(), Self::BlobStoreError> {
        let blob_id = blob_id(&bytes);
        if !self.has_blob(blob_id)? {
            let blake3: [u8; 32] = *blake3::hash(bytes).as_bytes();
            self.insert_blob_with(blob_id, blake3, bytes)?;
        }
        Ok(())
    }
    fn get_blob(&self, blob_id: GitId) -> Result<Option<Vec<u8>>, Self::BlobStoreError>;
    fn insert_blob_with(
        &mut self,
        blob_id: GitId,
        blake3: Blake3,
        bytes: &[u8],
    ) -> Result<(), Self::BlobStoreError>;
}

impl BlobStore for rusqlite::Connection {
    type BlobStoreError = rusqlite::Error;

    fn init_blob_cache(&mut self) -> Result<(), Self::BlobStoreError> {
        self.execute(
            r#"
            create table if not exists BlobStore(
                row_id integer primary key,
                bytes Blob not null,
                blob_id Blob
                    unique
                    generated always as( blob_id( bytes ) )
                    stored
                    check( length( blob_id ) = 20 ),
                blake3 Blob
                    unique
                    generated always as( blake3( bytes ) )
                    stored
                    check( length( blake3 ) = 32 ),
                length Integer
                    generated always as( length( bytes ) )
                    stored
                    check( length < 67108864 )
            ) strict
        "#,
            (),
        )?;

        self.execute_batch(
            r#"
            insert or ignore into BlobStore( bytes ) values( x'' );
            insert or ignore into BlobStore( bytes ) values( x'10' );
            insert or ignore into BlobStore( bytes ) values( zeroblob( 1024 ) );
            insert or ignore into BlobStore( bytes ) values( zeroblob( 1024 * 1024 ) );
        "#,
        )?;

        Ok(())
    }

    fn get_blob(&self, blob_id: GitId) -> Result<Option<Vec<u8>>, Self::BlobStoreError> {
        self.query_row(
            "select bytes from BlobStore where blob_id = ?",
            &[&blob_id],
            |row| row.get(0),
        )
        .optional()
    }

    fn insert_blob_with(
        &mut self,
        blob_id: GitId,
        blake3: Blake3,
        bytes: &[u8],
    ) -> Result<(), Self::BlobStoreError> {
        todo!()
    }
}

impl QueryCache for rusqlite::Connection {
    type QueryCacheError = rusqlite::Error;

    fn init_query_cache(&mut self) -> Result<(), Self::QueryCacheError> {
        self.execute(
            r#"
            create table if not exists QueryCache(
                request_blob_id Blob not null,
                response_blob_id Blob not null,
                timestamp Integer not null default( CURRENT_TIMESTAMP ),
                status Blob default( null ) check( status is null or length(status) <= 8 ),
                foreign key( request_blob_id ) references BlobStore( blob_id ),
                foreign key( response_blob_id ) references BlobStore( blob_id ),
                unique( request_blob_id, timestamp, status, response_blob_id )
            ) strict
        "#,
            (),
        )?;
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct HashMapCache {
    pub map: HashMap<GitId, Vec<u8>>,
}

impl BlobStore for HashMapCache {
    type BlobStoreError = Infallible;

    fn has_blob(&self, blob_id: GitId) -> Result<bool, Self::BlobStoreError> {
        Ok(self.map.contains_key(&blob_id))
    }

    fn get_blob(&self, blob_id: GitId) -> Result<Option<Vec<u8>>, Self::BlobStoreError> {
        Ok(self.map.get(&blob_id).cloned())
    }

    fn insert_blob_with(
        &mut self,
        blob_id: GitId,
        blake3: Blake3,
        bytes: &[u8],
    ) -> Result<(), Self::BlobStoreError> {
        self.map.insert(blob_id, bytes.to_vec());
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct NoCache;

impl BlobStore for NoCache {
    type BlobStoreError = Infallible;

    fn has_blob(&self, _blob_id: GitId) -> Result<bool, Self::BlobStoreError> {
        Ok(false)
    }

    fn get_blob(&self, _blob_id: GitId) -> Result<Option<Vec<u8>>, Self::BlobStoreError> {
        Ok(None)
    }

    fn insert_blob_with(
        &mut self,
        _blob_id: GitId,
        _blake3: Blake3,
        _bytes: &[u8],
    ) -> Result<(), Self::BlobStoreError> {
        Ok(())
    }
}

impl QueryCache for NoCache {
    type QueryCacheError = Infallible;
}
