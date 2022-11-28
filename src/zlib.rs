use {
    crate::{
        adler32, byte_buffer, default, generic::panic, write_deflate, DeflateMode, WriteAndSeek,
    },
    std::{future::Future, ops::Not, pin::Pin, task},
};

pub fn write_zlib(output: &mut impl WriteAndSeek, data: &[u8]) -> Result<usize, panic> {
    write_zlib {
        output,
        data,
        mode: default(),
    }
    .call()
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct write_zlib<'all, Output>
where Output: 'all + WriteAndSeek
{
    pub output: &'all mut Output,
    pub data: &'all [u8],
    pub mode: ZlibMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ZlibMode {
    /// deflate with a 32KiB LZ77 window
    Deflate32K { mode: DeflateMode },
}

impl Default for ZlibMode {
    fn default() -> Self {
        Self::Deflate32K {
            mode: DeflateMode::default(),
        }
    }
}
impl<WriteAndSeek> write_zlib<'_, WriteAndSeek>
where WriteAndSeek: self::WriteAndSeek
{
    pub fn call(&mut self) -> Result<usize, panic> {
        let Self { output, data, mode } = self;

        let before = output.offset();

        // zlib compression mode: deflate with 32KiB windows
        let cmf = 0b_0111_1000;
        output.write_all(&[cmf])?;
        // zlib flag bits: no preset dictionary, compression level 0
        let mut flg: u8 = 0b0000_0000;
        // zlib flag and check bits
        flg |= 0b1_1111 - ((((cmf as u16) << 8) | (flg as u16)) % 0b1_1111) as u8;
        output.write_all(&[flg])?;

        let mut buffer = byte_buffer();
        write_deflate(&mut buffer, data)?;

        output.write_all(&buffer.get_ref())?;

        // adler-32 checksum of the deflated data
        output.write_all(&adler32(buffer.get_ref()).to_le_bytes())?;

        Ok(output.offset() - before)
    }
}
