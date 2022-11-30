use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::ops::Index;
use std::ops::Range;

use smallvec::SmallVec;
use smol_str::SmolStr;
use tracing::warn;

use crate::generic::default;

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

#[cfg(test)]
#[test]
fn test() -> Result<(), panic> {
    let mut buffer = output_buffer();
    buffer.start("PNG", "file");

    buffer.start("PNG", "signature");
    buffer.write_all(b"\x89PNG\r")?;
    buffer.end("PNG", "signature");

    buffer.start("PNG", "image data");
    
    buffer.start("ZIP", "entry");
    
    buffer.end("ZIP", "file");
    buffer.end("PNG", "file");

    Ok(())
}

#[derive(Clone)]
pub enum BufferTag {
    Literal(&'static str),
    Dynamic(Arc<str>),
}

#[derive(Debug, Default, Clone)]
pub struct OutputBuffer {
    /// the actual data
    bytes: Vec<u8>,
    /// tracks of complete tags
    tag_tracks: BTreeMap<&'static str, Vec<TaggedRange>>,
    /// tracks of stacks of incomplete tags
    tag_stacks: BTreeMap<&'static str, Vec<TaggedRange>>,
}

impl OutputBuffer {
    pub fn new() -> Self {
        default()
    }

    pub fn push(&mut self, byte: u8) {
        self.bytes.push(byte);
    }

    /// Pushes a tag onto the stack of tags.
    pub fn start(&mut self, track: &'static str, tag: impl Into<SmolStr>) {
        todo!()
    }

    /// Pops a tag off the stack of tags, finalizes it at the current index, and pushes it onto the
    /// list of completed tags.
    pub fn end(&mut self, track: &'static str, tag: impl Into<SmolStr>) {
        todo!()
    }

    /// Defines a tag that will be applied to (only) the next byte.
    pub fn next(&mut self, track: &'static str, tag: impl Into<SmolStr>) {
        todo!()
    }

    pub fn offset(&self) -> usize {
        self.bytes.len()
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn tags(&self, track: &'static str) -> Option<&[TaggedRange]> {
        if !self.tag_stacks.values().all(|stack| stack.is_empty()) {
            warn!("tag track {track:?} requested but it still has incomplete tags in its stack.");
        }
        self.tag_tracks.get(track).map(|v| v.as_slice())
    }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
pub struct TaggedRange {
    /// The first index of the range (inclusive).
    start: usize,
    /// The last index of the range (exclusive).
    end: usize,
    /// The name/tag of the region.
    name: SmolStr,
    /// The child sub-regions of this region.
    children:  Vec<TaggedRange>,
}

impl TaggedRange {
    fn new(start: usize, name: impl Into<SmolStr>) -> Self {
        Self {
            start,
            end: usize::MAX,
            name: name.into(),
            children: Vec::new()
        }
    }

    fn finalize(mut self, end: usize) -> Self {
        assert_eq!(self.end, usize::MAX, "attempted to finalize a TaggedRange twice");
        assert!(end > self.start, "range must have nonzero length");
        self.end = end;
        self
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn name(&self) -> &SmolStr {
        &self.name
    }

    pub fn children(&self) -> &[TaggedRange] {
        &self.children
    }

    pub fn range(&self) -> Range<usize> {
        self.start..self.end
    }
}

impl Index<TaggedRange> for [u8] {
    type Output = [u8];

    fn index(&self, index: TaggedRange) -> &Self::Output {
        &self[index.start..=index.end]
    }
}

impl Index<TaggedRange> for OutputBuffer {
    type Output = [u8];

    fn index(&self, index: TaggedRange) -> &Self::Output {
        &self.bytes[index.start..=index.end]
    }
}


pub fn output_buffer() -> OutputBuffer {
    default()
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
