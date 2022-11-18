use std::fmt::Debug;
use std::fmt::Display;
use std::process::ExitCode;
use std::process::Termination;

use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use static_assertions::assert_impl_all;

pub fn default<T>() -> T
where T: Default {
    T::default()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
/// An uninhabited [`!`]-like "never" type, with trait implementations as needed
/// for convenience within this crate's types.
pub enum never {}

assert_impl_all!(never: Send, Sync, Serialize, DeserializeOwned);
assert_impl_all!(panic: Send, Sync, Serialize, DeserializeOwned);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
/// An uninhabited [`!`]-like "never" type that provides a panicking
/// implementation of `From` for any `Display + Debug` error type,
/// with trait implementations as needed for convenience within this crate's.
pub enum panic {}

impl<Err> From<Err> for panic
where Err: Display + Debug
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

impl Serialize for never {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        unreachable!()
    }
}

impl Serialize for panic {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        unreachable!()
    }
}

impl<'de> Deserialize<'de> for never {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        unreachable!()
    }
}

impl<'de> Deserialize<'de> for panic {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        unreachable!()
    }
}
