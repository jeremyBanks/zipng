pub(crate) mod reading;
pub(crate) mod writing;

pub mod palettes;

mod data;
mod dithering;
mod to_png;
mod sizes;

#[doc(inline)]
pub use self::{data::*, to_png::*, dithering::*, sizes::*};
