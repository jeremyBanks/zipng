#![allow(non_camel_case_types, unused)]

use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::process::ExitCode;
use std::process::Termination;

use serde::Deserialize;
use serde::Serialize;
use static_assertions::assert_impl_all;
use std::marker::PhantomData;
use serde::de;
use serde::de::Visitor;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[doc(hidden)]
pub enum never {}

assert_impl_all!(never: Send, Sync);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[doc(hidden)]
pub enum panic {}

assert_impl_all!(panic: Send, Sync);

impl<Err> From<Err> for panic
where
    Err: Display + Debug,
{
    #[track_caller]
    fn from(error: Err) -> Self {
        panic!("{error}")
    }
}

impl Default for never {
    fn default() -> Self {
        unreachable!()
    }
}

impl From<panic> for never {
    fn from(_: panic) -> never {
        unreachable!()
    }
}

impl From<never> for panic {
    fn from(_: never) -> panic {
        unreachable!()
    }
}

impl Termination for panic {
    fn report(self) -> ExitCode {
        unreachable!()
    }
}

impl Termination for never {
    fn report(self) -> ExitCode {
        unreachable!()
    }
}

#[allow(unused)]
#[doc(hidden)]
pub fn default<T>() -> T
where
    T: Default,
{
    T::default()
}

#[derive(Clone, Copy)]
#[doc(hidden)]
pub struct Ellipses;

impl Debug for Ellipses {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "…")
    }
}

impl Display for Ellipses {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "…")
    }
}

impl<T:?Sized> From<()> for PhantomType<T> {
    fn from(value: ()) -> Self {
        default()
    }
}

impl<T:?Sized> From<never> for PhantomType<T> {
    fn from(_: never) -> Self {
        unreachable!()
    }
}

impl<T:?Sized> From<panic> for PhantomType<T> {
    fn from(_: panic) -> Self {
        unreachable!()
    }
}

/// This is just a PhantomData wrapper for an invariant non-owned type with any lifetime,
/// implementing some traits for convenience.
#[derive(Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
struct PhantomType<T: ?Sized>(PhantomData<fn(T) -> T>);

impl<T:?Sized> Debug for PhantomType<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("…")
    }
}

impl<T:?Sized> Default for PhantomType<T> {
    fn default() -> Self {
        PhantomType(PhantomData)
    }
}

impl<T: ?Sized> Serialize for PhantomType<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_unit()
    }
}

impl<'de, T: ?Sized> Deserialize<'de> for PhantomType<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        deserializer.deserialize_unit(UnitVisitor)?;
        Ok(default())
    }
}


struct UnitVisitor;
impl<'de> Visitor<'de> for UnitVisitor {
    type Value = ();

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("nothing (zero-sized unit value)")
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error, {
        Ok(())
    }
}


assert_impl_all!(PhantomType<*const u8>: Send, Sync);
