use {
    crate::{adler32, panic, WriteAndSeek},
    std::ops::Not,
};

/// Writes the data with the headers and framing required for a deflate stream,
/// without performing any compression.
pub fn write_framed_as_deflate(
    output: &mut impl WriteAndSeek,
    data: &[u8],
) -> Result<usize, panic> {
    let before = output.offset();

    let chunks = data.chunks(0xFFFF);

    let count = chunks.len();
    for (index, chunk) in chunks.enumerate() {
        // deflate flag bits
        let is_last_chunk = index + 1 >= count;
        output.write_all(&[is_last_chunk.into()])?;
        // deflate block length
        output.write_all(&u16::try_from(chunk.len()).unwrap().to_le_bytes());
        // deflate block length check complement
        output.write_all(&u16::try_from(chunk.len()).unwrap().not().to_le_bytes());

        output.write_all(chunk);
    }

    Ok(output.offset() - before)
}
