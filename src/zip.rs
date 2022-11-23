use {
    crate::{never, panic},
    serde::{Deserialize, Serialize},
    std::io::{Read, Write},
};
mod configuration;
mod data;
mod to_zip;

use {
    crate::{zipng::writing::write_zip, WriteAndSeek},
    std::{io::Cursor, path::Path},
    tracing::{debug, instrument},
};

pub use self::{configuration::*, to_zip::*};

/// In-memory representation of a ZIP file's essential archive contents.
#[derive(Debug, Clone, Default)]
pub struct Zip {
    pub configuration: ZipConfiguration,
    pub files: Vec<(Vec<u8>, Vec<u8>)>,
}

impl Zip {
    /// Creates a new [`Zip`] from the given data.
    pub fn new(data: &impl ToZip) -> Self {
        data.to_zip().into_owned()
    }

    pub fn new_with_files(files: Vec<(Vec<u8>, Vec<u8>)>) -> Self {
        Self {
            files,
            ..Default::default()
        }
    }

    #[instrument(skip_all)]
    /// Creates a new [`Zip`] from the file or directory at the given path.
    pub fn new_from_path(path: impl AsRef<Path>) -> Result<Self, panic> {
        let path = path.as_ref();
        let mut files = Vec::<(Vec<u8>, Vec<u8>)>::new();

        let walkdir = walkdir::WalkDir::new(path);
        for entry in walkdir {
            let entry = entry?;
            if !entry.file_type().is_file() {
                continue;
            }
            // XXX: zip requires `/`-delimited paths, and if we're creating from
            // the filesystem directly I guess we're responsible for that,
            // although we don't want to touch users' own input if they're
            // manually specifying paths.
            let adding = entry.path().to_path_buf();
            debug!("adding {adding:?}");
            let contents = std::fs::read(adding)?;
            files.push((path.to_str().unwrap().as_bytes().to_vec(), contents));
        }

        Ok(Self::new_with_files(files))
    }

    /// Serializes this [`Zip`] as a ZIP archive file.
    pub fn write(&self, output: &mut impl WriteAndSeek) -> Result<usize, panic> {
        write_zip(
            output,
            self.files
                .iter()
                .map(|(a, b)| (a.as_slice(), b.as_slice()))
                .collect::<Vec<_>>()
                .as_slice(),
            &[],
        )
    }

    /// Deserializes a ZIP archive file into a [`Zip`].
    pub fn read(_input: &impl Read) -> Result<Self, panic> {
        unimplemented!()
    }

    /// Serializes this [`Zip`] into a byte vector as a ZIP archive file.
    pub fn write_vec(&self) -> Result<Vec<u8>, never> {
        let mut output = Cursor::new(Vec::new());
        self.write(&mut output)?;
        Ok(output.into_inner())
    }

    /// Deserialize a ZIP archive file into a [`Zip`] from a byte vector.
    pub fn read_slice(input: &[u8]) -> Result<Self, never> {
        Ok(Self::read(&input)?)
    }
}

impl Serialize for Zip {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        serializer.serialize_bytes(
            &self
                .write_vec()
                .expect("serializing Zip to bytes should not fail"),
        )
    }
}

impl<'de> Deserialize<'de> for Zip {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let bytes: &[u8] = serde_bytes::deserialize(deserializer)?;
        Self::read_slice(bytes).map_err(serde::de::Error::custom)
    }
}

fn zip<'files, Files>(files: Files) -> Vec<u8>
where Files: 'files + IntoIterator<Item = (&'files [u8], &'files [u8])> {
    let mut files: Vec<(&[u8], &[u8])> = files.into_iter().collect();
    files.sort_by_cached_key(|(path, body)| {
        (
            // file named "mimetype" goes first, for the sake of package formats including EPUB and
            // ODT.
            path != b"mimetype",
            // followed by any empty files, since they have no associated data and therefor weaker
            // alignment requirements, so we want to pack them all together.
            !body.is_empty(),
            // files before directories
            path.iter().filter(|&&b| b == b'/').count(),
            // then lexicographically by path
            *path,
            // and only then by body
            *body,
        )
    });
    let mut buffer = Vec::new();
    write_zip(&mut Cursor::new(&mut buffer), &files, b"").unwrap();
    buffer
}
