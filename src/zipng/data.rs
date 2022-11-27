use {
    crate::{
        byte_buffer,
        generic::{never, panic},
        palettes::RGB_256_COLOR_PALETTE_SIZE,
        write_framed_as_zlib, Png, ToZipng, WriteAndSeek, Zip, PNG_CHUNK_PREFIX_SIZE,
        PNG_CHUNK_WRAPPER_SIZE, PNG_HEADER_SIZE, ZIP_FILE_HEADER_EMPTY_SIZE,
    },
    std::io::{Cursor, Read},
    tracing::{instrument, warn},
};

/// In-memory representation of a ZIP file's archive contents and a PNG file's
/// image contents.
///
/// This is dumb what does this even mean?
///
/// This needs to be like a builder. Probably shouldn't contain a `Png`.
#[derive(Debug, Clone, Default)]
pub struct Zipng {
    pub zip: Zip,
    pub png: Png,
}

impl Zipng {
    /// Creates a new [`Zipng`] from the given data.
    pub fn new(data: &impl ToZipng) -> Self {
        data.to_zipng().into_owned()
    }
}

// Don't be excessively generic, buddy.
// You don't need this type. These are just methods on your Zip type.
// Keep minimal information on those types. Mostly arguments to DIFFERENT
// methods.

impl Zip {
    /// Serializes this [`Zip`] as a ZIP/PNG polyglot file.
    pub fn write_zipng(&self, mut output: &mut impl WriteAndSeek) -> Result<usize, panic> {
        if output.offset() != 0 {
            warn!(
                "PNG is being written at nonzero stream offset: {}",
                output.offset()
            );
        }

        let image_data_offset = PNG_HEADER_SIZE
            + (PNG_CHUNK_WRAPPER_SIZE + RGB_256_COLOR_PALETTE_SIZE)
            + (PNG_CHUNK_PREFIX_SIZE + ZIP_FILE_HEADER_EMPTY_SIZE);

        let mut image_data = byte_buffer();
        self.write(&mut image_data)?;

        write_framed_as_zlib(&mut output, image_data.get_mut())?;

        unimplemented!()
    }
}
