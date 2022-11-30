mod alignment;

use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;

pub use alignment::*;

use {
    crate::{
        generic::{default, panic},
    },
    core::fmt,
    derive_more::{Deref, From, TryInto},
    smallvec::SmallVec,
    kstring::KString,
    std::{
        borrow::Borrow,
        collections::{BTreeMap, BTreeSet},
        fmt::{Debug, Display},
        hash::{Hash, Hasher},
        io::{self, Cursor, Write},
        ops::{Deref, Index, Range},
        sync::Arc,
    },
    tracing::warn,
};

#[cfg(test)]
#[test]
fn test() -> Result<(), panic> {
    let mut buffer = output_buffer();
    buffer.start("PNG", "PNG");

    buffer.start("PNG", "signature");
    buffer.write_all(b"\x89PNG\r")?;
    buffer.end("PNG", "signature");

    buffer.start("PNG", "image data");

    buffer.start("ZIP", "ZIP");

    buffer.end("ZIP", "ZIP");
    buffer.end("PNG", "PNG");

    Ok(())
}

#[derive(Debug, Default, Clone)]
pub struct OutputBuffer {
    /// the actual data
    bytes: Vec<u8>,
    /// tracks of complete tags
    tag_tracks: BTreeMap<KString, Vec<TaggedRange>>,
    /// tracks of stacks of incomplete tags
    tag_stacks: BTreeMap<KString, Vec<TaggedRange>>,
}

impl OutputBuffer {
    pub fn new() -> Self {
        default()
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    pub fn get_ref(&self) -> &[u8] {
        self.as_ref()
    }

    pub fn get_mut(&mut self) -> &mut Vec<u8> {
        self.as_mut()
    }

    pub fn write_tagged(&mut self, data: &[u8], track: impl Into<KString>, tag: impl Into<KString>) {
        let track = track.into();
        let tag = tag.into();

        self.start(track.clone(), tag.clone());
        self.write_bytes(data);
        self.end(track, tag);
    }

    pub fn write_bytes(&mut self, data: &[u8]) {
        self.bytes.extend_from_slice(data);
    }

    /// Pushes a tag onto the stack of tags.
    pub fn start(&mut self, track: impl Into<KString>, tag: impl Into<KString>) {
        let track = track.into();
        let tag = tag.into();

        let start = self.offset();
        self.tag_stacks
            .entry(track)
            .or_default()
            .push(TaggedRange::new(start, tag));
    }

    /// Pops a tag off the stack of tags, finalizes it at the current index, and
    /// pushes it onto the list of completed tags.
    pub fn end(&mut self, track: impl Into<KString>, tag: impl Into<KString>) {
        let track = track.into();
        let tag = tag.into();

        let end = self.offset();
        let stack = self
            .tag_stacks
            .get_mut(&track)
            .expect("attempting to close a tag on a track that doesn't exist");
        let range = stack
            .pop()
            .expect("attempted to close a tag but none were open");
        assert_eq!(range.name, tag, "attempted to close unexpected tag");
        self.tag_tracks
            .entry(track)
            .or_default()
            .push(range.finalize(end));
    }

    pub fn offset(&self) -> usize {
        self.bytes.len()
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn tags(&self, track: impl Into<KString>) -> Option<&[TaggedRange]> {
        let track = track.into();

        if !self.tag_stacks.values().all(|stack| stack.is_empty()) {
            warn!("tag track {track:?} requested but it still has incomplete tags in its stack.");
        }
        self.tag_tracks.get(&track).map(|v| v.as_slice())
    }
}

impl IntoIterator for OutputBuffer {
    type Item = u8;
    type IntoIter = std::vec::IntoIter<u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.bytes.into_iter()
    }
}

impl From<OutputBuffer> for Vec<u8> {
    fn from(buffer: OutputBuffer) -> Self {
        buffer.into_bytes()
    }
}

impl AsRef<[u8]> for OutputBuffer {
    fn as_ref(&self) -> &[u8] {
        self.bytes.as_slice()
    }
}

impl AsRef<Vec<u8>> for OutputBuffer {
    fn as_ref(&self) -> &Vec<u8> {
        &self.bytes
    }
}

impl AsMut<[u8]> for OutputBuffer {
    fn as_mut(&mut self) -> &mut [u8] {
        self.bytes.as_mut_slice()
    }
}

impl AsMut<Vec<u8>> for OutputBuffer {
    fn as_mut(&mut self) -> &mut Vec<u8> {
        &mut self.bytes
    }
}

impl Offset for OutputBuffer {
    fn offset(&mut self) -> usize {
        OutputBuffer::offset(self)
    }

    fn len(&mut self) -> usize {
        OutputBuffer::len(self)
    }
}

impl Write for OutputBuffer {
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        self.bytes.write(buf)
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        self.bytes.flush()
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
pub struct TaggedRange {
    /// The first index of the range (inclusive).
    start: usize,
    /// The last index of the range (exclusive).
    end: usize,
    /// The name/tag of the region.
    name: KString,
    /// The child sub-regions of this region.
    children: Vec<TaggedRange>,
}

impl TaggedRange {
    fn new(start: usize, name: impl Into<KString>) -> Self {
        Self {
            start,
            end: usize::MAX,
            name: name.into(),
            children: Vec::new(),
        }
    }

    fn finalize(mut self, end: usize) -> Self {
        assert_eq!(
            self.end,
            usize::MAX,
            "attempted to finalize a TaggedRange twice"
        );
        if end == self.start {
            warn!("finalizing an empty TaggedRange");
        }
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

    pub fn name(&self) -> &KString {
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

pub trait OutputRead: Read + Offset {}
impl<T> OutputRead for T where T: Read + Offset {}


pub trait InputWrite: Write + Offset {}
impl<T> InputWrite for T where T: Write + Offset {}

pub trait Offset {
    fn offset(&mut self) -> usize;

    fn len(&mut self) -> usize;
}

impl<T> Offset for T where T: Seek
{
    fn offset(&mut self) -> usize {
        let Ok(position) = self.stream_position() else {
            return usize::MAX;
        };
        let Ok(position) = usize::try_from(position) else {
            return usize::MAX;
        };
        position
    }

    fn len(&mut self) -> usize {
        let old_pos = self.stream_position().unwrap_or(u64::MAX);
        let len = self.seek(SeekFrom::End(0)).unwrap_or(u64::MAX);
        if old_pos != len {
            self.seek(SeekFrom::Start(old_pos)).ok();
        }
        len.try_into().unwrap_or(usize::MAX)
    }
}
