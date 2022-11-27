use std::future::Future;
use std::pin::Pin;
use std::task;

use crate::{adler32, byte_buffer, generic::panic, write_framed_as_deflate, WriteAndSeek};

/// Writes the data with the headers and framing required for a zlib stream,
/// without performing any compression.
pub fn write_framed_as_zlib(output: &mut impl WriteAndSeek, data: &[u8]) -> Result<usize, panic> {
    WriteFramedAsZlib { output, data }.execute()
}

pub struct WriteFramedAsZlib<'output, 'data, Output: 'output + WriteAndSeek> {
    output: &'output mut Output,
    data: &'data [u8],
}

impl<WriteAndSeek: crate::WriteAndSeek> Future for WriteFramedAsZlib<'_, '_, WriteAndSeek> {
    type Output = Result<usize, panic>;

    fn poll(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        
        task::Poll::Ready(self.execute())
    }
}

impl<WriteAndSeek: self::WriteAndSeek> WriteFramedAsZlib<'_, '_, WriteAndSeek> {
    pub fn execute(self) -> Result<usize, panic> {
        let Self { output, data } = self;

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
        write_framed_as_deflate(&mut buffer, data)?;
    
        output.write_all(&buffer.get_ref())?;
    
        // adler-32 checksum of the deflated data
        output.write_all(&adler32(buffer.get_ref()).to_le_bytes())?;
    
        Ok(output.offset() - before)
    }
}
