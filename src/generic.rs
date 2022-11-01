use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;

use serde::Deserialize;
use serde::Serialize;

pub fn default<T>() -> T
where
    T: Default,
{
    T::default()
}

#[allow(non_camel_case_types)]
pub type never = core::convert::Infallible;

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
