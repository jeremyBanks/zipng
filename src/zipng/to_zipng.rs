use {
    crate::{default, Png, Zip, Zipng},
    std::borrow::Cow,
};

/// A [`Zipng`], a [`Zip`], or a [`Png`].
pub trait ToZipng: Clone {
    fn to_zipng(&self) -> Cow<Zipng>;
}

impl ToZipng for Zipng {
    fn to_zipng(&self) -> Cow<Zipng> {
        Cow::Borrowed(self)
    }
}

impl ToZipng for Png {
    fn to_zipng(&self) -> Cow<Zipng> {
        Cow::Owned(Zipng {
            zip: default(),
            png: self.clone(),
        })
    }
}

impl ToZipng for Zip {
    fn to_zipng(&self) -> Cow<Zipng> {
        Cow::Owned(Zipng {
            zip: self.clone(),
            png: default(),
        })
    }
}
