#![allow(non_camel_case_types, unused)]

use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::process::ExitCode;
use std::process::Termination;

use databake::Bake;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[doc(hidden)]
pub enum never {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[doc(hidden)]
pub enum panic {}

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

impl Bake for panic {
    fn bake(&self, ctx: &databake::CrateEnv) -> databake::TokenStream {
        unreachable!()
    }
}

impl Bake for never {
    fn bake(&self, ctx: &databake::CrateEnv) -> databake::TokenStream {
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
