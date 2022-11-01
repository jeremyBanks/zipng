use std::any::Any;
use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::process::ExitCode;
use std::process::Termination;

use miette::Diagnostic;
use miette::IntoDiagnostic;
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;

pub fn default<T>() -> T
where
    T: Default,
{
    T::default()
}

#[allow(non_camel_case_types)]
pub type never = core::convert::Infallible;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum panic {}

#[track_caller]
pub fn panic<Err>(error: Err) -> panic
where
    Err: Any + Send + Debug,
{
    panic!("{error:?}")
}

impl Termination for panic {
    fn report(self) -> ExitCode {
        ExitCode::FAILURE
    }
}

impl<Err> From<Err> for panic
where
    Err: Any + Send + Debug,
{
    #[track_caller]
    fn from(error: Err) -> Self {
        panic(error)
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct Ellipses;

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
