use {
    crate::{adler32, generic::default, panic, WriteAndSeek},
    std::ops::Not,
};

pub fn write_deflate(output: &mut impl WriteAndSeek, data: &[u8]) -> Result<usize, panic> {
    write_deflate {
        output,
        data,
        mode: default(),
    }
    .call()
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct write_deflate<'all, Output>
where Output: 'all + WriteAndSeek
{
    pub output: &'all mut Output,
    pub data: &'all [u8],
    pub mode: DeflateMode,
}

impl<'all, Output> write_deflate<'all, Output>
where Output: 'all + WriteAndSeek
{
    pub fn call(&mut self) -> Result<usize, panic> {
        let Self {
            output,
            data,
            mode: DeflateMode::NoCompression { max_block_size },
        } = self;

        let before = output.offset();

        let chunks = data.chunks((*max_block_size).into());

        let count = chunks.len();
        for (index, chunk) in chunks.enumerate() {
            // deflate flag bits
            let is_last_chunk = index + 1 >= count;
            output.write_all(&[is_last_chunk.into()])?;
            // deflate block length
            output.write_all(&u16::try_from(chunk.len()).unwrap().to_le_bytes())?;
            // deflate block length check complement
            output.write_all(&u16::try_from(chunk.len()).unwrap().not().to_le_bytes())?;

            output.write_all(chunk)?;
        }

        Ok(output.offset() - before)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DeflateMode {
    /// No compression. Only use "Non-compressed block (BTYPE=00)" blocks.
    NoCompression {
        /// The maximum number of bytes to include in each block.
        ///
        /// The maximum size of the block in the output stream will be this
        /// value plus five bytes for the block headers.
        max_block_size: u16,
    },
}

impl Default for DeflateMode {
    fn default() -> Self {
        Self::NoCompression {
            max_block_size: u16::MAX,
        }
    }
}
