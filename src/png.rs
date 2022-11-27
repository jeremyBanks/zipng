pub(crate) mod reading;
pub(crate) mod writing;

pub mod palettes;

mod data;
mod dithering;
mod sizes;
mod to_png;

#[doc(inline)]
pub use self::{data::*, dithering::*, sizes::*, to_png::*};
