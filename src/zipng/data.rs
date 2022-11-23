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
        {
            // what do we want?
            // png header needs to come first
        }

        if output.offset() != 0 {
            warn!(
                "PNG is being written at nonzero stream offset: {}",
                output.offset()
            );
        }

        // in order to write the header, we need to know the image metadata.
        // it has a fixed length, which can help with figuring out the alignment, I
        // guess. Well, I guess we can generate the zip data as a single block,
        // align it as a whole(!?), and work on that? no. maybe. we'll see.

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
