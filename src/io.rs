use {
    crate::{generic::panic, WriteAndSeek},
    core::fmt,
    derive_more::{Deref, From, TryInto},
    std::{
        borrow::Borrow,
        fmt::{Debug, Display},
        hash::{Hash, Hasher},
        io::{Cursor, Write},
        ops::Deref,
        sync::Arc,
    },
};

#[derive(Clone)]
pub enum BufferTag {
    Literal(&'static str),
    Dynamic(Arc<str>),
}

mod _buffer_tag {
    use super::*;

    impl BufferTag {
        pub fn literal(s: &'static str) -> Self {
            Self::Literal(s)
        }

        pub fn dynamic(s: impl AsRef<str>) -> Self {
            Self::Dynamic(s.as_ref().to_string().into_boxed_str().into())
        }
    }

    impl AsRef<str> for BufferTag {
        fn as_ref(&self) -> &str {
            match self {
                BufferTag::Literal(s) => s,
                BufferTag::Dynamic(s) => s,
            }
        }
    }

    impl Debug for BufferTag {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            Debug::fmt(self.as_ref(), f)
        }
    }

    impl Display for BufferTag {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            Display::fmt(self.as_ref(), f)
        }
    }

    impl Deref for BufferTag {
        type Target = str;

        fn deref(&self) -> &Self::Target {
            self.as_ref()
        }
    }

    impl Borrow<str> for BufferTag {
        fn borrow(&self) -> &str {
            self.as_ref()
        }
    }

    impl PartialEq for BufferTag {
        fn eq(&self, other: &Self) -> bool {
            self.as_ref() == other.as_ref()
        }
    }

    impl Eq for BufferTag {}

    impl PartialOrd for BufferTag {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.as_ref().partial_cmp(other.as_ref())
        }
    }

    impl Ord for BufferTag {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.as_ref().cmp(other.as_ref())
        }
    }

    impl Hash for BufferTag {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.as_ref().hash(state)
        }
    }
}

/// Buffer will implement Read and Write and Seek, but it will also allow nested
/// tagging of the bytes.
pub struct Buffer {
    index: usize,
    bytes: Vec<u8>,
}

pub fn byte_buffer() -> Cursor<Vec<u8>> {
    Cursor::new(Vec::new())
}

/// Alignment direction, possible for rendered text or binary data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Align {
    /// align to the start / left / top / beginning
    Start,
    /// align to the middle / center
    Middle,
    /// align to the end / right / bottom
    End,
}

impl Align {
    /// align to the left / top / beginning / start
    pub const Left: Align = Align::Start;
    /// align to the top / beginning / start / left
    pub const Top: Align = Align::Start;
    /// align to the beginning / start / left / top
    pub const Beginning: Align = Align::Start;
    /// align to the center / middle
    pub const Center: Align = Align::Middle;
    /// align to the right / bottom / end
    pub const Right: Align = Align::End;
    /// align to the bottom / end / right
    pub const Bottom: Align = Align::End;
}

/// Writes `bytes` to `output` with as much padding as necessary to align
/// ensure that it lines up with the next multiple of the computed alignment
/// in the output stream.
///
/// The default alignment is
/// [`output.len().next_power_of_two()`][u64::next_power_of_two], clamped by
/// `minimum_alignment` (defaults to `1_024`) and `maximum_alignment` (defaults
/// to `1_048_576`).
///
/// `direction` determines how the output is supposed to line up
/// with the alignment interval: the default ([`Align::Start`]) places the
/// beginning of the output at the alignment interval, while [`Align::End`]
/// places the end of the output at the alignment interval.
///
/// `skip_padding_below_length` defines a size below which no padding will be
/// written for the value. The default is `1`, which means that zero-length
/// values will not be padded, but everything else will.
///
/// `fully_padded` determines whether the `bytes` must be fully padded within
/// their alignment intervals (`true`, the default), or whether it's okay for
/// other values to be packed into the unused space (`false`).
///
/// `padding_bytes` determines the bytes used to pad the output, defaulting to
/// all-zeros.
#[allow(clippy::too_many_arguments)]
pub fn write_aligned<'a>(
    output: &mut impl WriteAndSeek,
    bytes: impl AsRef<[u8]>,
    minimum_alignment: impl Into<Option<usize>>,
    maximum_alignment: impl Into<Option<usize>>,
    direction: impl Into<Option<Align>>,
    skip_padding_below_length: impl Into<Option<usize>>,
    fully_padded: impl Into<Option<bool>>,
    padding_bytes: impl Into<Option<&'a [u8]>>,
) -> Result<usize, panic> {
    let bytes = bytes.as_ref();

    let skip_padding_below_length = skip_padding_below_length.into().unwrap_or(1);

    let minimum_alignment = minimum_alignment.into().unwrap_or(1_024);
    let maximum_alignment = maximum_alignment.into().unwrap_or(1_048_576);

    let direction = direction.into().unwrap_or(Align::Start);

    let fully_padded = fully_padded.into().unwrap_or(true);

    let padding_bytes = padding_bytes
        .into()
        .filter(|b| !b.is_empty())
        .unwrap_or(b"\x00");

    return write_aligned(
        output,
        bytes,
        minimum_alignment,
        maximum_alignment,
        direction,
        skip_padding_below_length,
        fully_padded,
        padding_bytes,
    );
    fn write_aligned(
        output: &mut impl WriteAndSeek,
        bytes: &[u8],
        minimum_alignment: usize,
        maximum_alignment: usize,
        _direction: Align,
        skip_padding_below_length: usize,
        _fully_padded: bool,
        _padding_bytes: &[u8],
    ) -> Result<usize, panic> {
        let previous_offset = output.offset();

        let natural_alignment = bytes.len().next_power_of_two();
        let _alignment = natural_alignment.clamp(minimum_alignment, maximum_alignment);

        // TODO: all of the things

        let final_offset = output.offset();
        Ok(final_offset - previous_offset)
    }
}

/// Writes `bytes` to `buffer`, padded with trailing zeroes to the next multiple
/// of `alignment`. Returns the range that `bytes` was written to in `buffer`,
/// excluding the padding.
pub(crate) fn write_aligned_pad_end(
    output: &mut impl WriteAndSeek,
    bytes: &[u8],
    alignment: usize,
) -> Result<usize, panic> {
    let index_before_data = output.offset();

    output.write_all(bytes)?;

    let index_after_data = output.offset();

    if index_after_data % alignment != 0 {
        let padding = alignment - (index_after_data % alignment);
        for _ in 0..padding {
            output.write_all(&[0])?;
        }
    }

    let _index_after_padding = output.offset();

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
    let index_before_padding = output.offset();
    let unpadded_index_after_data = index_before_padding + bytes.len();
    if unpadded_index_after_data % alignment != 0 {
        let padding = alignment - (unpadded_index_after_data % alignment);
        for _ in 0..padding {
            output.write_all(&[0])?;
        }
    }
    let index_before_data = output.offset();
    output.write_all(bytes)?;
    let index_after_data = output.offset();
    Ok((index_before_data..index_after_data).len())
}
