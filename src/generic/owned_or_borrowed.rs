use std::borrow::Borrow;
use std::fmt::Debug;

/// Simple borrowed-or-owned enum without any type constraints except for
/// `Sized`. Only neccessary because `Cow<T>` requires `T: Clone`.
pub enum Borrowable<'a, T: 'a + Sized> {
    Shared(&'a T),
    Exclusive(&'a mut T),
    Owned(T),
}

impl<'a, T: 'a> Borrowable<'a, T> {
    pub fn take(self) -> Option<T> {
        match self {
            Borrowable::Owned(value) => Some(value),
            _ => None,
        }
    }

    pub fn borrow_mut(&mut self) -> Option<&mut T> {
        match self {
            Borrowable::Shared(_) => None,
            Borrowable::Exclusive(value) => Some(value),
            Borrowable::Owned(value) => Some(value),
        }
    }
}

impl<'a, T: 'a> Borrow<T> for Borrowable<'a, T> {
    fn borrow(&self) -> &T {
        match self {
            Borrowable::Shared(x) => x,
            Borrowable::Exclusive(x) => x,
            Borrowable::Owned(x) => x,
        }
    }
}

impl<'a, T: 'a> AsRef<T> for Borrowable<'a, T> {
    fn as_ref(&self) -> &T {
        self.borrow()
    }
}

impl<'a, T: 'a> From<T> for Borrowable<'a, T> {
    fn from(value: T) -> Self {
        Borrowable::Owned(value)
    }
}

impl<'a, T: 'a> From<&T> for Borrowable<'a, T> {
    fn from(value: &T) -> Self {
        Borrowable::Shared(value)
    }
}

impl<'a, T: 'a> From<&mut T> for Borrowable<'a, T> {
    fn from(value: &mut T) -> Self {
        Borrowable::Exclusive(value)
    }
}

// add TryInto, and qualified delegation for the common derivable traits

impl<'a, T: 'a> Debug for Borrowable<'a, T>
where T: Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Shared(value) => f.write_str("&")?,
            Self::Exclusive(value) => f.write_str("&mut ")?,
            Self::Owned(value) => {},
        }
        self.as_ref().fmt(f)
    }
}
