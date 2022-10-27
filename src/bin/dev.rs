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

/// A reference to a Blob. If the blob is at least 32 bytes in length,
/// this is the 32-byte blake3 hash of that blob. If it is less than 32
/// bytes in length, the value is left intact.
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlobRef {
    length: usize,
    bytes: [u8; 32],
}

pub fn blob(slice: impl AsRef<[u8]>) -> BlobRef {
    BlobRef::new(slice)
}

impl BlobRef {
    pub fn new(slice: impl AsRef<[u8]>) -> Self {
        let slice = slice.as_ref();
        let slice_length = slice.len();
        let mut bytes = [0u8; blake3::OUT_LEN];
        if slice_length >= blake3::OUT_LEN {
            bytes.copy_from_slice(blake3::hash(slice).as_bytes());
        } else {
            bytes[..slice_length].copy_from_slice(slice);
        }
        let length = slice_length.min(32);
        Self { length, bytes }
    }

    pub fn as_inline(&self) -> Option<&[u8]> {
        if self.length < blake3::OUT_LEN {
            Some(self.as_ref())
        } else {
            None
        }
    }

    pub fn as_inline_ascii(&self) -> Option<&str> {
        let bytes = self.as_inline()?;
        if bytes
            .iter()
            .all(|b| matches!(b, b'\n' | b'\r' | b'\t' | b' '..=b'~'))
        {
            Some(str::from_utf8(bytes).unwrap())
        } else {
            None
        }
    }
}

impl AsRef<[u8]> for BlobRef {
    fn as_ref(&self) -> &[u8] {
        &self.bytes[..self.length]
    }
}

impl Debug for BlobRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        Ok(if let Some(ascii) = self.as_inline_ascii() {
            write!(f, "b{ascii:?}")?;
        } else {
            write!(f, "0x")?;
            for byte in self.as_ref() {
                write!(f, "{:02X}", byte)?;
            }
        })
    }
}

impl Display for BlobRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        Debug::fmt(self, f)
    }
}

#[test]
fn test_blob_ref() {
    macro_rules! cases {
        ($($input:expr => $to_string:expr, $to_json:expr);+ $(;)?) => {
            $(
                assert_eq!($to_string, blob($input).to_string());
                assert_eq!($to_json, ::serde_json::to_string(&blob($input)).unwrap());
            )+
        };
    };
    cases! {
        "" => r#"b"""#, r#""""#;
        [] => r#"b"""#, r#""""#;
        [0] => "0x00", "[0]";
        [1, 2, 3] => "0x010203", "[1,2,3]";
        "[]" => r#"b"[]""#, r#""[]""#;
        "{}" => r#"b"{}""#, r#""{}""#;
        "alpine glacial foreland wurm" => "b\"alpine glacial foreland wurm\"", "\"alpine glacial foreland wurm\"";
        "alfa bravo charlie delta echo foxtrot golf hotel india juliett kilo lima mike november oscar papa quebec romeo sierra tango uniform whiskey x-ray yankee zulu stop" => "0x62EB8E3ECF44B3E16B4ABDD5B67672BEDCD2B51826AC585F3B6D4AE988082DA4", "[98,235,142,62,207,68,179,225,107,74,189,213,182,118,114,190,220,210,181,24,38,172,88,95,59,109,74,233,136,8,45,164]";
    }
}

impl Serialize for BlobRef {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if let Some(ascii) = self.as_inline_ascii() {
            serializer.serialize_str(ascii)
        } else {
            serializer.serialize_bytes(self.as_ref())
        }
    }
}

impl<'input> Deserialize<'input> for BlobRef {
    fn deserialize<D: Deserializer<'input>>(deserializer: D) -> Result<Self, D::Error> {
        return deserializer.deserialize_bytes(Visitor);

        struct Visitor;
        impl<'input> serde::de::Visitor<'input> for Visitor {
            type Value = BlobRef;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("up to 32 bytes, or up to 31 ascii characters")
            }

            fn visit_bytes<E: serde::de::Error>(self, v: &[u8]) -> Result<Self::Value, E> {
                let mut bytes = [0; blake3::OUT_LEN];
                let length = v.len();
                bytes[..length].copy_from_slice(v);
                Ok(BlobRef { length, bytes })
            }

            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                self.visit_bytes(v.as_bytes())
            }
        }
    }
}
