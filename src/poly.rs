use crate::{ToPng, ToZip};

pub trait ToZipng: ToZip + ToPng {}
impl<T> ToZipng for T where T: ToZip + ToPng {}
