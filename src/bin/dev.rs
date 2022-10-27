#![cfg_attr(debug_assertions, allow(unused))]
use std::collections::HashMap;
use std::convert::Infallible;
use std::env;
use std::fmt::Debug;
use std::format as f;
use std::hash::Hasher;

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
use twox_hash::Xxh3Hash64;
use typenum::U20;

#[derive(Debug, From, AsRef, AsMut)]
pub struct Connection {
    connection: rusqlite::Connection,
}

const APPLICATION_ID: i32 = 0x_F_1C_15_00;

impl Connection {
    pub fn new(mut connection: rusqlite::Connection) -> Self {
        fn initialize_connection(
            connection: &mut rusqlite::Connection,
        ) -> Result<(), rusqlite::Error> {
            info!("Initializing connection...");
            connection.pragma_update(None, "auto_vacuum", "full")?;
            connection.pragma_update(None, "foreign_keys", true)?;
            connection.pragma_update(None, "synchronous", "normal")?;
            connection.pragma_update(None, "temp_store", "memory")?;
            connection.pragma_update(None, "secure_delete", true)?;
            connection.pragma_update(None, "cache_size", 1024)?;

            initialize_connection_functions(connection)?;
            initialize_connection_extensions(connection)?;

            Ok(())
        }
        fn initialize_connection_functions(
            connection: &mut rusqlite::Connection,
        ) -> Result<(), rusqlite::Error> {
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
                |ctx| Ok(blake3(ctx.get_raw(0).as_bytes()?)),
            )?;

            connection.create_scalar_function(
                "xxh3",
                1,
                FunctionFlags::SQLITE_DETERMINISTIC,
                |ctx| Ok(xxh3(ctx.get_raw(0).as_bytes()?)),
            )?;
            Ok(())
        }
        fn initialize_connection_extensions(
            connection: &mut rusqlite::Connection,
        ) -> Result<(), rusqlite::Error> {
            Ok(unsafe {
                let guard = LoadExtensionGuard::new(&connection)?;
                connection.load_extension("sqlite_zstd", None)?;
            })
        }

        fn get_versions(
            connection: &mut rusqlite::Connection,
        ) -> Result<(i32, i32, i32), rusqlite::Error> {
            connection.query_row(
                "
                select
                    application_id ,
                    user_version ,
                    schema_version
                from
                    pragma_application_id join
                    pragma_user_version join
                    pragma_schema_version
            ",
                (),
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
            )
        }

        let (application_id, user_version, schema_version) = get_versions(&mut connection).unwrap();

        if application_id == 0 {
            info!("assuming database requires initialization because application_id is 0.");
            if schema_version != 0 {
                warn!("schema_version is already {schema_version}.");
            }
            if user_version != 0 {
                warn!("user_version is already {user_version}.");
            }
            initialize_database(&mut connection).unwrap();
        } else if application_id != APPLICATION_ID {
            error!("database has an unexpected application_id: {application_id:08X}");
        }
        fn initialize_database(
            connection: &mut rusqlite::Connection,
        ) -> Result<(), rusqlite::Error> {
            info!("Initializing database...");
            connection.pragma_update(None, "journal_mode", "wal")?;
            connection.pragma_update(None, "page_size", 64 * 1024)?;
            connection.pragma_update(None, "user_version", 1)?;
            connection.pragma_update(None, "application_id", APPLICATION_ID)?;
            Ok(())
        }

        initialize_connection(&mut connection).unwrap();

        migrate_database(&mut connection).unwrap();

        fn migrate_database(connection: &mut rusqlite::Connection) -> Result<(), rusqlite::Error> {
            loop {
                let (application_id, user_version, schema_version) =
                    get_versions(connection).unwrap();
                match user_version {
                    1 => {
                        info!("migration 1: adding BlobStore");
                        connection
                            .execute_batch(
                                r#"
                            create table BlobStore(
                                bytes Blob not null,
                                row_id integer primary key,
                                prefix Blob,
                                length Integer,
                                blob_id Blob,
                                blake3 Blob,
                                xxh3 Integer,
                                unique( blob_id ),
                                unique( blake3 ),
                                unique( xxh3, row_id ),
                                unique( prefix, row_id )
                            ) strict;

                            create trigger BlobStoreComputedColumns
                                after insert on BlobStore begin
                                    update BlobStore
                                        set
                                            length = length( new.bytes ),
                                            blob_id = blob_id( new.bytes ),
                                            blake3 = blake3( new.bytes ),
                                            xxh3 = xxh3( new.bytes ),
                                            prefix = substr( new.bytes, 1, 16 )
                                        where row_id = new.row_id;
                                end;

                            select zstd_enable_transparent( '{
                                "table": "BlobStore",
                                "column": "bytes",
                                "compression_level": 21,
                                "dict_chooser": "''BlobStore''"
                            }');
                        "#,
                            )
                            .unwrap();
                    },
                    2 => {
                        info!("migration 2: seeding BlobStore");
                        connection
                            .execute_batch(r#"
                                    insert into BlobStore( bytes ) values( zeroblob(0) );
                                    insert into BlobStore( bytes ) values( zeroblob(1) );
                                    insert into BlobStore( bytes ) values( zeroblob(2) );
                                    insert into BlobStore( bytes ) values( zeroblob(4) );
                                    insert into BlobStore( bytes ) values( zeroblob(8) );
                                    insert into BlobStore( bytes ) values( zeroblob(16) );
                                    insert into BlobStore( bytes ) values( zeroblob(32) );
                                    insert into BlobStore( bytes ) values( zeroblob(64) );
                                    insert into BlobStore( bytes ) values( cast( '{}' as blob ) );
                                    insert into BlobStore( bytes ) values( cast( '[]' as blob ) );
                                    insert into BlobStore( bytes ) values( cast( '\n' as blob ) );
                                    insert into BlobStore( bytes ) values( cast( '\r\n' as blob ) );
                                    insert into BlobStore( bytes ) values( cast( '0' as blob ) );
                                    insert into BlobStore( bytes ) values( cast( '1' as blob ) );
                                    insert into BlobStore( bytes ) values( cast( '0.0' as blob ) );
                                    insert into BlobStore( bytes ) values( cast( '1.0' as blob ) );
                                    insert into BlobStore( bytes ) values( cast( 'false' as blob ) );
                                    insert into BlobStore( bytes ) values( cast( 'true' as blob ) );
                                    insert into BlobStore( bytes ) values( zeroblob(1024) );
                                    insert into BlobStore( bytes ) values( zeroblob(32 * 1024) );
                                    insert into BlobStore( bytes ) values( zeroblob(1024 * 1024) );
                                    insert into BlobStore( bytes ) values( zeroblob(32 * 1024 * 1024) );
                                "#,
                            )
                            .unwrap();
                    },
                    3 => {
                        info!("migration 3: adding QueryCache");
                        connection.execute(
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
                        ).unwrap();
                    },

                    4 => break,

                    other => panic!("database has an unexpected user_version: {other}"),
                }
                connection
                    .pragma_update(None, "user_version", user_version + 1)
                    .unwrap();
            }
            Ok(())
        }

        optimize_database(&mut connection).unwrap();

        fn optimize_database(connection: &mut rusqlite::Connection) -> Result<(), rusqlite::Error> {
            info!("optimizing database");
            connection.execute_batch(
                r#"
                    select zstd_incremental_maintenance(null, 0.9375);
                    vacuum;
                    analyze;
                "#,
            )?;
            Ok(())
        }

        {
            let mut q = connection
                .prepare("select max(length) from BlobStore")
                .unwrap();
            let result = q.query_row((), |row| Ok(format!("{:?}", row.get_ref(0))));
            dbg!(result);
        }

        Self { connection }
    }

    pub fn open_in_memory() -> Self {
        Self::new(rusqlite::Connection::open_in_memory().unwrap())
    }

    pub fn open(path: &str) -> Result<Self, rusqlite::Error> {
        Ok(Self::new(rusqlite::Connection::open(path)?))
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

    println!("{:02X?}", blob_id(b""));

    Ok(())
}

pub type BlobId = [u8; 20];
pub type Blake3 = [u8; 32];
pub type Xxh3 = i64;

/// BLAKE3 cryptographic hash
fn blake3(bytes: &[u8]) -> [u8; 32] {
    *blake3::hash(bytes).as_bytes()
}

/// Git's blob object SHA-1 pseudo-cryptographic hash
fn blob_id(bytes: &[u8]) -> [u8; 20] {
    sha1::Sha1::new()
        .chain_update("blob")
        .chain_update(" ")
        .chain_update(bytes.len().to_string())
        .chain_update([0x00])
        .chain_update(&bytes)
        .finalize()
        .into()
}

/// XXH3's 64-bit non-cryptographic hash
///
/// We return as an i64 instead of u64 because that's what SQLite supports
/// directly.
fn xxh3(bytes: &[u8]) -> i64 {
    let mut hasher = twox_hash::Xxh3Hash64::with_seed(0);
    hasher.write(bytes);
    hasher.finish() as i64
}
