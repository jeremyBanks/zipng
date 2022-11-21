pub(crate) mod reading;
pub(crate) mod writing;

pub mod palettes;

mod data;
mod to_png;
#[doc(inline)]
pub use self::{data::*, to_png::*};
