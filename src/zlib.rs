use crate::WriteAndSeek;
use crate::adler32;
use crate::byte_buffer;
use crate::generic::panic;
use crate::write_framed_as_deflate;



/// Writes the data with the headers and framing required for a zlib stream,
/// without performing any compression.
pub fn write_framed_as_zlib(output: &mut impl WriteAndSeek, data: &[u8]) -> Result<usize, panic> {
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
    output.write_all(&adler32(buffer.get_ref()).to_le_bytes());

    Ok(output.offset() - before)
}
