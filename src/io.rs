mod alignment;

use std::collections::HashMap;

pub use alignment::*;
use tracing::trace;
use {
    crate::generic::{default, panic},
    core::fmt,
    derive_more::{Deref, From, TryInto},
    expect_test::expect,
    kstring::KString,
    smallvec::SmallVec,
    std::{
        borrow::Borrow,
        collections::{BTreeMap, BTreeSet},
        fmt::{Debug, Display},
        hash::{Hash, Hasher},
        io::{self, Cursor, Read, Seek, SeekFrom, Write},
        iter,
        ops::{Add, AddAssign, Deref, DerefMut, Index, Range},
        sync::Arc,
    },
    tracing::warn,
};

/// In-memory output buffer supporting multiple [overlapping hierarchical markup
/// tracks][1].
///
/// Can be concatenated with other `OutputBuffer`s while preserving tags.
///
/// [1]: https://en.wikipedia.org/wiki/Overlapping_markup
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

    pub fn tagged(
        &mut self,
        track: impl Into<KString>,
        tag: impl Into<KString>,
    ) -> InOutputBufferTag<'_> {
        let track = track.into();
        let tag = tag.into();
        self.start(track.clone(), tag.clone());
        InOutputBufferTag {
            buffer: Some(self),
            track,
            tag,
        }
    }

    pub fn extend<Data>(&mut self, data: Data)
    where OutputBuffer: AddAssign<Data> {
        *self += data;
    }

    /// Concatenates the contents of other onto self.
    /// Closed tags are copied over, nested under the current tag if one is open
    /// on the corresponding track, and with their offsets adjusted
    /// appropriately. Unclosed tags are silently discarded.
    pub fn concat(&mut self, other: &Self) {
        let offset = self.bytes.len();
        self.bytes.extend_from_slice(&other.bytes);

        for (other_track, other_tags) in &other.tag_tracks {
            let track = self.tag_tracks.entry(other_track.clone()).or_default();
            let stack = self.tag_stacks.entry(other_track.clone()).or_default();
            let parent = stack.last_mut().map(|t| &mut t.children).unwrap_or(track);

            for tag in other_tags {
                parent.push(tag.clone() + offset);
            }
        }
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
    /// then adds it as a child of the next tag on the stack, if one exists,
    /// or else adds it to the list of complete tags.
    pub fn end(&mut self, track: impl Into<KString>, tag: impl Into<KString>) {
        let track = track.into();
        let tag = tag.into();

        let end = self.offset();
        let stack = self.tag_stacks.get_mut(&track).unwrap();
        let mut range = stack.pop().unwrap();
        range.end = end;
        if let Some(parent) = stack.last_mut() {
            parent.children.push(range);
        } else {
            self.tag_tracks.entry(track).or_default().push(range);
        }
    }

    pub fn offset(&self) -> usize {
        self.bytes.len()
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn tracks(&self) -> impl Iterator<Item = &KString> {
        self.tag_tracks.keys()
    }

    pub fn walk_tags<'a>(
        &'a self,
        track: impl Into<KString>,
    ) -> Box<dyn 'a + Iterator<Item = OutputBufferWalkEntry>> {
        let track = track.into();
        let mut tags: Vec<OutputBufferWalkEntry> = self
            .tag_tracks
            .get(&track)
            .iter()
            .flat_map(|t| t.iter())
            .flat_map(|t| t.iter())
            .flat_map(|t| {
                [
                    OutputBufferWalkEntry {
                        index: t.start,
                        other_index: usize::MAX - t.end,
                        close_else_open: false,
                        track: track.clone(),
                        tag: t.name.clone(),
                    },
                    OutputBufferWalkEntry {
                        index: t.end,
                        other_index: usize::MAX - t.start,
                        close_else_open: true,
                        track: track.clone(),
                        tag: t.name.clone(),
                    },
                ]
            })
            .collect();
        tags.sort();
        Box::new(tags.into_iter())
    }

    pub fn root_tags(&self, track: impl Into<KString>) -> Option<&[TaggedRange]> {
        let track = track.into();

        if !self.tag_stacks.values().all(|stack| stack.is_empty()) {
            warn!("tag track {track:?} requested but it still has incomplete tags in its stack.");
        }
        self.tag_tracks
            .get(&track)
            .map(|v| v.as_slice())
            .filter(|v| !v.is_empty())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OutputBufferWalkEntry {
    index: usize,
    other_index: usize,
    close_else_open: bool,
    track: KString,
    tag: KString,
}

impl Display for OutputBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn write_bytes(f: &mut fmt::Formatter, bytes: &[u8]) -> fmt::Result {
            for byte in bytes {
                match byte {
                    b'<' => write!(f, "&lt;")?,
                    b'>' => write!(f, "&gt;")?,
                    b'&' => write!(f, "&amp;")?,
                    0 => write!(f, "&#0;")?,
                    b if b.is_ascii_graphic() => write!(f, "{}", (*b as char))?,
                    b => write!(f, "&#x{b:02X};")?,
                }
            }

            Ok(())
        }

        let mut tags = Vec::new();
        for track in self.tracks() {
            tags.extend(self.walk_tags(track.clone()));
        }
        tags.sort();

        let mut index = 0;

        let mut indentation_by_track = HashMap::new();
        let mut indentation_total = 0;
        for tag in tags {
            let indentation: &mut usize = indentation_by_track.entry(tag.track.clone()).or_default();
            
            let before = &self.bytes[index..tag.index];
            if !before.is_empty() {
                write!(f, "{:indent$}", "", indent = indentation_total * 4)?;
                write_bytes(f, before)?;
                index = tag.index;
                write!(f, "\n")?;
            }

            if tag.close_else_open {
                *indentation -= 1;
                indentation_total -= 1;
            }

            write!(f, "{:indent$}", "", indent = *indentation * 4)?;

            if !tag.close_else_open {
                *indentation += 1;
                indentation_total += 1;
                write!(f, "<{}:{}>", tag.track, tag.tag)?;
            } else {
                write!(f, "</{}:{}>", tag.track, tag.tag)?;
            }
            write!(f, "\n")?;
        }
        write_bytes(f, &self.bytes[index..])?;

        Ok(())
    }
}

impl AddAssign<OutputBuffer> for OutputBuffer {
    fn add_assign(&mut self, other: OutputBuffer) {
        self.concat(&other);
    }
}

impl AddAssign<&OutputBuffer> for OutputBuffer {
    fn add_assign(&mut self, other: &OutputBuffer) {
        self.concat(other);
    }
}

impl AddAssign<&[u8]> for OutputBuffer {
    fn add_assign(&mut self, other: &[u8]) {
        self.bytes.extend_from_slice(other);
    }
}

impl<const N: usize> AddAssign<&[u8; N]> for OutputBuffer {
    fn add_assign(&mut self, other: &[u8; N]) {
        self.bytes.extend_from_slice(other.as_slice());
    }
}

impl AddAssign<OutputBuffer> for &mut OutputBuffer {
    fn add_assign(&mut self, other: OutputBuffer) {
        self.concat(&other);
    }
}

impl AddAssign<&OutputBuffer> for &mut OutputBuffer {
    fn add_assign(&mut self, other: &OutputBuffer) {
        self.concat(other);
    }
}

impl AddAssign<&[u8]> for &mut OutputBuffer {
    fn add_assign(&mut self, other: &[u8]) {
        self.bytes.extend_from_slice(other);
    }
}

impl<const N: usize> AddAssign<&[u8; N]> for &mut OutputBuffer {
    fn add_assign(&mut self, other: &[u8; N]) {
        self.bytes.extend_from_slice(other.as_slice());
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

    pub fn iter<'a>(&'a self) -> Box<dyn 'a + Iterator<Item = &'a TaggedRange>> {
        // XXX: wtf are you doing fix this
        let mut items = vec![self];
        for child in &self.children {
            items.extend(child.iter());
        }
        Box::new(items.into_iter())
    }
}

impl Add<usize> for TaggedRange {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self {
            start: self.start + rhs,
            end: self.end + rhs,
            name: self.name,
            children: self.children,
        }
    }
}

impl AddAssign<usize> for TaggedRange {
    fn add_assign(&mut self, rhs: usize) {
        self.start += rhs;
        self.end += rhs;
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

#[derive(Debug)]
pub struct InOutputBufferTag<'buffer> {
    buffer: Option<&'buffer mut OutputBuffer>,
    track: KString,
    tag: KString,
}

impl<'buffer> InOutputBufferTag<'buffer> {
    pub fn closed(mut self) -> &'buffer mut OutputBuffer {
        self.buffer.take().unwrap()
    }
}

impl Deref for InOutputBufferTag<'_> {
    type Target = OutputBuffer;

    fn deref(&self) -> &Self::Target {
        self.buffer.as_ref().unwrap()
    }
}

impl DerefMut for InOutputBufferTag<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.buffer.as_mut().unwrap()
    }
}

impl Drop for InOutputBufferTag<'_> {
    fn drop(&mut self) {
        if let Some(buffer) = &mut self.buffer {
            buffer.end(self.track.clone(), self.tag.clone());
        }
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

impl<T> Offset for T
where T: Seek
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

#[cfg(test)]
#[test]
fn test_output_buffer() -> Result<(), panic> {
    crate::dev::init!();

    let mut buffer = output_buffer();

    {
        let mut buffer = buffer.tagged("PNG", "PNG");

        buffer.extend(b"\x89PNG\r");

        let mut buffer = buffer.tagged("PNG", "IHDR");

        buffer.extend(b"\x00\x00\x00\rIHDR");

        let mut buffer = buffer.closed();

        buffer.extend(b"test");

        buffer.start("PNG", "signature");

        buffer.end("PNG", "signature");

        buffer.start("PNG", "image data");

        buffer.start("ZIP", "ZIP");

        let mut sub = output_buffer();
        sub.tagged("PNG", "sub").extend(b"\x90PNG\r");
        buffer += sub;

        buffer.end("PNG", "image data");

        buffer.end("ZIP", "ZIP");
        buffer.end("PNG", "PNG");
    }

    expect![[r#"
        Some(
            [
                TaggedRange {
                    start: 17,
                    end: 22,
                    name: "ZIP",
                    children: [],
                },
            ],
        )
    "#]]
    .assert_debug_eq(&buffer.root_tags("ZIP"));

    expect![[r#"
        Some(
            [
                TaggedRange {
                    start: 0,
                    end: 22,
                    name: "PNG",
                    children: [
                        TaggedRange {
                            start: 5,
                            end: 22,
                            name: "IHDR",
                            children: [
                                TaggedRange {
                                    start: 17,
                                    end: 17,
                                    name: "signature",
                                    children: [],
                                },
                                TaggedRange {
                                    start: 17,
                                    end: 22,
                                    name: "image data",
                                    children: [
                                        TaggedRange {
                                            start: 17,
                                            end: 22,
                                            name: "sub",
                                            children: [],
                                        },
                                    ],
                                },
                            ],
                        },
                    ],
                },
            ],
        )
    "#]]
    .assert_debug_eq(&buffer.root_tags("PNG"));

    expect![[r#"
        <PNG:PNG>
            &#x89;PNG&#x0D;
            <PNG:IHDR>
                &#x00;&#x00;&#x00;&#x0D;IHDRtest
                <PNG:image data>
                    <PNG:sub>
        <ZIP:ZIP>
                        <PNG:signature>
                        </PNG:signature>
                            &#x90;PNG&#x0D;
                    </PNG:image data>
                </PNG:sub>
        </ZIP:ZIP>
            </PNG:IHDR>
        </PNG:PNG>
    "#]]
    .assert_eq(&buffer.to_string());

    Ok(())
}
