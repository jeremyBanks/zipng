
use crate::PNG_CHUNK_PREFIX_SIZE;
use crate::PNG_CHUNK_WRAPPER_SIZE;
use crate::PNG_HEADER_SIZE;
use crate::ZIP_FILE_HEADER_EMPTY_SIZE;
use crate::palettes::RGB_256_COLOR_PALETTE_SIZE;

use {
    crate::{
        generic::{never, panic},
        Png, ToZipng, WriteAndSeek, Zip,
    },
    std::io::{Cursor, Read},
    tracing::{instrument, warn},
};

/// In-memory representation of a ZIP file's archive contents and a PNG file's
/// image contents.
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

    #[instrument(skip_all)]
    /// Serializes this [`Zipng`] as a ZIP/PNG polyglot file.
    pub fn write(&self, output: &mut impl WriteAndSeek) -> Result<usize, panic> {
        if output.offset() != 0 {
            warn!(
                "PNG is being written at nonzero stream offset: {}",
                output.offset()
            );
        }

        // I guess the chunk-alignment can be delayed or even ditched for this operation.
        // We can probably layer it on later if we want to.
        // First, just make it work?

        let image_data_offset =
            PNG_HEADER_SIZE +
            (PNG_CHUNK_WRAPPER_SIZE + RGB_256_COLOR_PALETTE_SIZE) + 
            (PNG_CHUNK_PREFIX_SIZE + ZIP_FILE_HEADER_EMPTY_SIZE);
        


        unimplemented!()
    }

    /// Serializes this [`Zip`] into a byte vector as a ZIP/PNG polyglot file.
    pub fn write_vec(&self) -> Result<Vec<u8>, never> {
        let mut output = Cursor::new(Vec::new());
        self.write(&mut output)?;
        Ok(output.into_inner())
    }

    #[instrument(skip_all)]
    /// Deserializes a ZIP/PNG polyglot file into a [`Zipng`].
    pub fn read(input: &impl Read) -> Result<Self, panic> {
        let zip = Zip::read(input)?;
        let png = Png::read(input)?;
        Ok(Self { zip, png })
    }

    /// Deserialize a ZIP/PNG polyglot file into a [`Zip`] from a byte vector.
    pub fn read_slice(input: &[u8]) -> Result<Self, never> {
        Ok(Self::read(&input)?)
    }
}
