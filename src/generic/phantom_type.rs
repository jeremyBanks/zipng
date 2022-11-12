use std::cmp::Ordering;
use std::fmt;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;

use serde::de;
use serde::Deserialize;
use serde::Serialize;
use static_assertions::assert_impl_all;

/// This is a convenience wrapper for `PhantomData<fn(T) -> T>`, which
/// seems to be the right way to defined a `PhantomData` without affecting
/// either the borrow checker (lifetimes) or the drop checker (ownership,
/// borrowing).
pub struct Type<T: ?Sized>(PhantomData<fn(T) -> T>);

impl<T: ?Sized> Copy for Type<T> {}

assert_impl_all!(
  Type<(*const u8, dyn Debug)>:
    Copy,
    Send,
    Sync,
    Default,
    Sized,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize<'static>,
    From<()>,
    Into<()>,
);

impl<T: ?Sized> Clone for Type<T> {
    fn clone(&self) -> Self {
        Type(PhantomData)
    }
}

impl<T: ?Sized> PartialEq for Type<T> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<T: ?Sized> Eq for Type<T> {}

impl<T: ?Sized> PartialOrd for Type<T> {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}

impl<T: ?Sized> Ord for Type<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
    }
}

impl<T: ?Sized> Hash for Type<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {}
}

impl<T: ?Sized> Type<T> {
    pub fn new() -> Self {
        Type(PhantomData)
    }
}

impl<T: ?Sized> Debug for Type<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("👻")
    }
}

impl<T: ?Sized> Default for Type<T> {
    fn default() -> Self {
        Type(PhantomData)
    }
}

impl<T: ?Sized> From<()> for Type<T> {
    fn from(_: ()) -> Self {
        Type(PhantomData)
    }
}

impl<T: ?Sized> From<Type<T>> for () {
    fn from(_: Type<T>) -> Self {}
}

impl<T: ?Sized> Serialize for Type<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        serializer.serialize_unit()
    }
}

impl<'de, T: ?Sized> Deserialize<'de> for Type<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        deserializer.deserialize_unit(UnitVisitor)?;
        Ok(Type(PhantomData))
    }
}

struct UnitVisitor;
impl<'de> de::Visitor<'de> for UnitVisitor {
    type Value = ();

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("nothing (zero-sized unit value)")
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where E: de::Error {
        Ok(())
    }
}
