use {
    crate::{generic::panic, WriteAndSeek},
    std::io::Write,
};

/// Writes `bytes` to `buffer`, padded with trailing zeroes to the next multiple
/// of `alignment`. Returns the range that `bytes` was written to in `buffer`,
/// excluding the padding.
pub(crate) fn write_aligned_pad_end(
    output: &mut impl WriteAndSeek,
    bytes: &[u8],
    alignment: usize,
) -> Result<usize, panic> {
    let index_before_data = output.stream_position()? as usize;

    output.write_all(bytes)?;

    let index_after_data = output.stream_position()? as usize;

    if index_after_data % alignment != 0 {
        let padding = alignment - (index_after_data % alignment);
        for _ in 0..padding {
            output.write_all(&[0])?;
        }
    }

    let _index_after_padding = output.stream_position()? as usize;

    Ok((index_before_data..index_after_data).len())
}

/// Writes `bytes` to `buffer`, padded with leading zeroes to the next multiple
/// of `alignment`. Returns the range that `bytes` was written to in `buffer`,
/// excluding the padding.
pub(crate) fn write_aligned_pad_start(
    output: &mut impl WriteAndSeek,
    bytes: &[u8],
    alignment: usize,
) -> Result<usize, panic> {
    let index_before_padding = output.stream_position()? as usize;
    let unpadded_index_after_data = index_before_padding + bytes.len();
    if unpadded_index_after_data % alignment != 0 {
        let padding = alignment - (unpadded_index_after_data % alignment);
        for _ in 0..padding {
            output.write_all(&[0])?;
        }
    }
    let index_before_data = output.stream_position()? as usize;
    output.write_all(bytes)?;
    let index_after_data = output.stream_position()? as usize;
    Ok((index_before_data..index_after_data).len())
}
