#![cfg_attr(debug_assertions, allow(unused))]
use std::collections::HashMap;
use std::convert::Infallible;
use std::fmt::Debug;

use digest::generic_array::GenericArray;
use digest::Digest;
use serde::Deserialize;
use serde::Serialize;
use sha1;
use typenum::U20;

pub fn main() -> Result<(), eyre::Report> {
    let mut cache = rusqlite::Connection::open_in_memory()?;
    cache.init_blob_cache()?;
    cache.init_query_cache()?;

    println!("{:02X?}", blob_git_id(b""));

    Ok(())
}

pub type GitId = [u8; 20];

fn blob_git_id(bytes: &[u8]) -> GitId {
    sha1::Sha1::new()
        .chain_update("blob")
        .chain_update(" ")
        .chain_update(bytes.len().to_string())
        .chain_update([0x00])
        .chain_update(&bytes)
        .finalize()
        .into()
}

trait QueryCache: BlobCache {
    type QueryCacheError;
    fn init_query_cache(&mut self) -> Result<(), Self::QueryCacheError> {
        Ok(())
    }
}

trait BlobCache {
    type BlobCacheError;
    fn init_blob_cache(&mut self) -> Result<(), Self::BlobCacheError> {
        Ok(())
    }
    fn has_blob(&self, git_id: GitId) -> Result<bool, Self::BlobCacheError> {
        Ok(self.get_blob(git_id)?.is_none())
    }
    fn insert_blob(&mut self, bytes: &[u8]) -> Result<(), Self::BlobCacheError> {
        let git_id = blob_git_id(&bytes);
        if !self.has_blob(git_id)? {
            self.insert_blob_at(git_id, bytes)?;
        }
        Ok(())
    }
    fn get_blob(&self, git_id: GitId) -> Result<Option<Vec<u8>>, Self::BlobCacheError>;
    fn insert_blob_at(&mut self, git_id: GitId, bytes: &[u8]) -> Result<(), Self::BlobCacheError>;
}

impl BlobCache for rusqlite::Connection {
    type BlobCacheError = rusqlite::Error;

    fn init_blob_cache(&mut self) -> Result<(), Self::BlobCacheError> {
        self.execute(
            r#"
            create table if not exists BlobCache(
                blob_id Blob primary key check( length( git_id ) = 20 ),
                bytes Blob not null check( length( bytes ) <= 67108864 ),
            ) strict;
        "#,
            (),
        )?;
        Ok(())
    }

    fn get_blob(&self, git_id: GitId) -> Result<Option<Vec<u8>>, Self::BlobCacheError> {
        todo!()
    }

    fn insert_blob_at(&mut self, git_id: GitId, bytes: &[u8]) -> Result<(), Self::BlobCacheError> {
        todo!()
    }
}

impl QueryCache for rusqlite::Connection {
    type QueryCacheError = rusqlite::Error;

    fn init_query_cache(&mut self) -> Result<(), Self::QueryCacheError> {
        self.execute(r#"
            create table if not exists QueryCache(
                request_blob_id Blob not null foreign key( request_blob_id ) references BlobCache( git_id ),
                response_blob_id Blob not null foreign key( response_blob_id ) references BlobCache( git_id ),
                timestamp Integer not null default( CURRENT_TIMESTAMP ),
                status Blob default( null ) check( status is null or length(status) <= 8 ),
                unique( request_blob_id, timestamp, status, response_blob_id),
            ) strict;
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

impl BlobCache for HashMapCache {
    type BlobCacheError = Infallible;

    fn has_blob(&self, git_id: GitId) -> Result<bool, Self::BlobCacheError> {
        Ok(self.map.contains_key(&git_id))
    }

    fn get_blob(&self, git_id: GitId) -> Result<Option<Vec<u8>>, Self::BlobCacheError> {
        Ok(self.map.get(&git_id).cloned())
    }

    fn insert_blob_at(&mut self, git_id: GitId, bytes: &[u8]) -> Result<(), Self::BlobCacheError> {
        self.map.insert(git_id, bytes.to_vec());
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct NoCache;

impl BlobCache for NoCache {
    type BlobCacheError = Infallible;

    fn has_blob(&self, _git_id: GitId) -> Result<bool, Self::BlobCacheError> {
        Ok(false)
    }

    fn get_blob(&self, _git_id: GitId) -> Result<Option<Vec<u8>>, Self::BlobCacheError> {
        Ok(None)
    }

    fn insert_blob_at(
        &mut self,
        _git_id: GitId,
        _bytes: &[u8],
    ) -> Result<(), Self::BlobCacheError> {
        Ok(())
    }
}

impl QueryCache for NoCache {
    type QueryCacheError = Infallible;
}
