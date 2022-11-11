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
pub struct PhantomType<T: ?Sized>(PhantomData<fn(T) -> T>);

impl<T: ?Sized> Copy for PhantomType<T> {}

assert_impl_all!(
  PhantomType<(*const u8, dyn Debug)>:
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

impl<T: ?Sized> Clone for PhantomType<T> {
    fn clone(&self) -> Self {
        PhantomType(PhantomData)
    }
}

impl<T: ?Sized> PartialEq for PhantomType<T> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<T: ?Sized> Eq for PhantomType<T> {}

impl<T: ?Sized> PartialOrd for PhantomType<T> {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}

impl<T: ?Sized> Ord for PhantomType<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
    }
}

impl<T: ?Sized> Hash for PhantomType<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {}
}

impl<T: ?Sized> PhantomType<T> {
    pub fn new() -> Self {
        PhantomType(PhantomData)
    }
}

impl<T: ?Sized> Debug for PhantomType<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("👻")
    }
}

impl<T: ?Sized> Default for PhantomType<T> {
    fn default() -> Self {
        PhantomType(PhantomData)
    }
}

impl<T: ?Sized> From<()> for PhantomType<T> {
    fn from(_: ()) -> Self {
        PhantomType(PhantomData)
    }
}

impl<T: ?Sized> From<PhantomType<T>> for () {
    fn from(_: PhantomType<T>) -> Self {}
}

impl<T: ?Sized> Serialize for PhantomType<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_unit()
    }
}

impl<'de, T: ?Sized> Deserialize<'de> for PhantomType<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_unit(UnitVisitor)?;
        Ok(PhantomType(PhantomData))
    }
}

struct UnitVisitor;
impl<'de> de::Visitor<'de> for UnitVisitor {
    type Value = ();

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("nothing (zero-sized unit value)")
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(())
    }
}
