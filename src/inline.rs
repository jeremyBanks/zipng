use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::fmt::Debug;
use std::hash::Hash;

use static_assertions::assert_impl_all;
use thiserror::Error;

use crate::generic::default;
use crate::Blip;

trait Value: Copy + Default + Eq + Ord + Debug + Hash + Debug + 'static {}
impl<T> Value for T where T: Copy + Default + Eq + Ord + Debug + Hash + Debug + 'static + Sized {}

assert_impl_all!(u8: Value);
assert_impl_all!(char: Value);
assert_impl_all!((u32, u32): Value);
// assert_impl_all!(Blip<str>: Value);

pub struct Array<const Capacity: usize, T: Value> {
    length: usize,
    buffer: [T; Capacity],
}

impl<const Capacity: usize, T: Value> Array<Capacity, T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn try_from_slice(slice: &[T]) -> Result<Self, CapacityExceeded> {
        slice.try_into()
    }

    pub fn try_from_vec(slice: Vec<T>) -> Result<Self, CapacityExceeded> {
        slice.as_slice().try_into()
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub const fn capacity(&self) -> usize {
        Capacity
    }

    pub fn iter(&self) -> <Self as IntoIterator>::IntoIter {
        self.into_iter()
    }
}

impl<const Capacity: usize, T: Value> Default for Array<Capacity, T> {
    fn default() -> Self {
        Self {
            length: default(),
            buffer: [default(); Capacity],
        }
    }
}

impl<const Capacity: usize, T: Value> AsRef<[T]> for Array<Capacity, T> {
    fn as_ref(&self) -> &[T] {
        &self.buffer[0..self.length]
    }
}

impl<const Capacity: usize, T: Value> AsMut<[T]> for Array<Capacity, T> {
    fn as_mut(&mut self) -> &mut [T] {
        &mut self.buffer[0..self.length]
    }
}

impl<const Capacity: usize, T: Value> Borrow<[T]> for Array<Capacity, T> {
    fn borrow(&self) -> &[T] {
        self.as_ref()
    }
}

impl<const Capacity: usize, T: Value> BorrowMut<[T]> for Array<Capacity, T> {
    fn borrow_mut(&mut self) -> &mut [T] {
        self.as_mut()
    }
}

impl<const Capacity: usize, T: Value> Hash for Array<Capacity, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state)
    }
}
impl<const Capacity: usize, T: Value> Eq for Array<Capacity, T> {}
impl<const Capacity: usize, T: Value> PartialEq for Array<Capacity, T> {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref().eq(other.as_ref())
    }
}
impl<const Capacity: usize, T: Value> Ord for Array<Capacity, T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_ref().cmp(other.as_ref())
    }
}
impl<const Capacity: usize, T: Value> PartialOrd for Array<Capacity, T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.as_ref().partial_cmp(other.as_ref())
    }
}
impl<const Capacity: usize, T: Value> Debug for Array<Capacity, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_ref().fmt(f)
    }
}
impl<const Capacity: usize, T: Value> Clone for Array<Capacity, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<const Capacity: usize, T: Value> Copy for Array<Capacity, T> {}

impl<const Capacity: usize, T: Value> From<heapless::Vec<T, Capacity>> for Array<Capacity, T> {
    fn from(value: heapless::Vec<T, Capacity>) -> Self {
        value.as_slice().try_into().unwrap()
    }
}

#[derive(Debug, Error)]
#[error("Array capacity exceeded")]
pub struct CapacityExceeded;

impl<const Capacity: usize, T: Value> TryFrom<&[T]> for Array<Capacity, T> {
    type Error = CapacityExceeded;

    fn try_from(value: &[T]) -> Result<Self, Self::Error> {
        let length = value.len();
        if length <= Capacity {
            let mut buffer = [default(); Capacity];
            buffer[..length].copy_from_slice(value);
            Ok(Self { buffer, length })
        } else {
            Err(CapacityExceeded)
        }
    }
}

impl<const Capacity: usize, T: Value> From<[T; Capacity]> for Array<Capacity, T> {
    fn from(value: [T; Capacity]) -> Self {
        Self {
            length: Capacity,
            buffer: value,
        }
    }
}

impl<const Capacity: usize, T: Value> IntoIterator for Array<Capacity, T> {
    type Item = T;
    type IntoIter = std::iter::Take<std::array::IntoIter<T, Capacity>>;

    fn into_iter(self) -> Self::IntoIter {
        self.buffer.into_iter().take(self.length)
    }
}
