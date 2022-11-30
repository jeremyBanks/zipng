use {
    crate::{never, panic},
    serde::{Deserialize, Serialize},
    std::io::{Read, Write},
};
mod configuration;
mod data;
mod sizes;
mod to_zip;
mod write_zip;

use {
    self::write_zip::write_zip,
    crate::InputWrite,
    std::{io::Cursor, path::Path},
    tracing::{debug, instrument},
};

pub use self::{configuration::*, data::*, sizes::*, to_zip::*};

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
