use std::borrow::Borrow;
use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;
use std::ops::Deref;

/// Simple borrowed-or-owned enum without any type constraints except for
/// `Sized`. Only neccessary because `Cow<T>` requires `T: Clone`.
pub enum Borrowable<'a, T: 'a + ?Sized> {
    Shared(&'a T),
    Leased(&'a mut T),
    Owned(T),
}

impl<'a, T: 'a> Borrowable<'a, T> {
    pub fn to_owned(self) -> T
    where T: Clone {
        match self {
            Borrowable::Shared(t) => t.clone(),
            Borrowable::Leased(t) => t.clone(),
            Borrowable::Owned(t) => t,
        }
    }

    pub fn try_as_mut(&mut self) -> Option<&mut T> {
        match self {
            Borrowable::Shared(_) => None,
            Borrowable::Leased(t) => Some(t),
            Borrowable::Owned(t) => Some(t),
        }
    }

    pub fn owned(self) -> Option<T> {
        match self {
            Borrowable::Owned(value) => Some(value),
            _ => None,
        }
    }

    pub fn leased(&mut self) -> Option<&mut T> {
        match self {
            Borrowable::Leased(value) => Some(value),
            _ => None,
        }
    }

    pub fn shared(&self) -> Option<&T> {
        match self {
            Borrowable::Shared(value) => Some(value),
            _ => None,
        }
    }
}

impl<'a, T: 'a> Borrow<T> for Borrowable<'a, T> {
    fn borrow(&self) -> &T {
        match self {
            Borrowable::Shared(x) => x,
            Borrowable::Leased(x) => x,
            Borrowable::Owned(x) => x,
        }
    }
}

impl<'a, T: 'a> AsRef<T> for Borrowable<'a, T> {
    fn as_ref(&self) -> &T {
        self.borrow()
    }
}

impl<'a, T: 'a> Deref for Borrowable<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
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
        Borrowable::Leased(value)
    }
}

// add TryInto, and qualified delegation for the common derivable traits
// we don't need to go as far as the real Cow. Although I suppose

impl<'a, T: 'a> Debug for Borrowable<'a, T>
where T: Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Shared(value) => f.write_str("&")?,
            Self::Leased(value) => f.write_str("&mut ")?,
            Self::Owned(value) => {},
        }
        self.as_ref().fmt(f)
    }
}

impl<'a, T: 'a> Clone for Borrowable<'a, T>
where T: Clone
{
    fn clone(&self) -> Self {
        self.to_owned()
    }
}

impl<'a, T: 'a> Hash for Borrowable<'a, T>
where T: Hash
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state)
    }
}

impl<'a, T: 'a> Ord for Borrowable<'a, T>
where T: Ord
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_ref().cmp(other.as_ref())
    }
}

impl<'a, T: 'a> Eq for Borrowable<'a, T> where T: Eq {}

impl<'a, T: 'a> PartialEq for Borrowable<'a, T>
where T: PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        self.as_ref().eq(other.as_ref())
    }
}

impl<'a, T: 'a> PartialOrd for Borrowable<'a, T>
where T: PartialOrd
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.as_ref().partial_cmp(other.as_ref())
    }
}

impl<'a, T: 'a> Default for Borrowable<'a, T>
where T: Default
{
    fn default() -> Self {
        Self::Owned(T::default())
    }
}

impl<'a, T: 'a> Display for Borrowable<'a, T>
where T: Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_ref().fmt(f)
    }
}
