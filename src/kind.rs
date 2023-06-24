use core::ops::Deref;

use self::RefKind::{Mut, Ref};

/// Provides different kinds of reference:
/// [immutable](Ref) or [mutable](Mut) one.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RefKind<'a, T>
where
    T: ?Sized + 'a,
{
    /// Immutable kind of reference.
    Ref(&'a T),
    /// Mutable kind of reference.
    Mut(&'a mut T),
}

impl<'a, T> RefKind<'a, T>
where
    T: ?Sized + 'a,
{
    /// Checks if [`RefKind`] contains immutable reference.
    #[inline]
    pub fn is_ref(&self) -> bool {
        matches!(self, Ref(_))
    }

    /// Checks if [`RefKind`] contains mutable reference.
    #[inline]
    pub fn is_mut(&self) -> bool {
        matches!(self, Mut(_))
    }

    /// Returns an immutable reference from the [`RefKind`].
    #[inline]
    pub fn get_ref(&self) -> &T {
        self
    }

    /// Returns [`Some`] with a mutable reference from the struct
    /// or [`None`] if contained reference is immutable.
    #[inline]
    pub fn get_mut(&mut self) -> Option<&mut T> {
        match self {
            Ref(_) => None,
            Mut(unique) => Some(unique),
        }
    }

    /// Converts [`RefKind`] into immutable reference with the lifetime of the owner,
    /// consuming the `self` value.
    #[inline]
    pub fn into_ref(self) -> &'a T {
        match self {
            Ref(shared) => shared,
            Mut(unique) => unique,
        }
    }

    /// Returns [`Some`] with a mutable reference with the lifetime of the owner
    /// or [`None`] if contained reference is immutable, consuming the `self` value.
    #[inline]
    pub fn into_mut(self) -> Option<&'a mut T> {
        match self {
            Ref(_) => None,
            Mut(unique) => Some(unique),
        }
    }

    /// Returns the contained [`Ref`] value, consuming the `self` value.
    ///
    /// # Panics
    ///
    /// Panics if the value is a [`Mut`].
    #[inline]
    #[track_caller]
    pub fn unwrap_ref(self) -> &'a T {
        match self {
            Ref(shared) => shared,
            Mut(_) => panic!("called `RefKind::unwrap_ref()` on a `RefKind::Mut` value"),
        }
    }

    /// Returns the contained [`Mut`] value, consuming the `self` value.
    ///
    /// # Panics
    ///
    /// Panics if the value is a [`Ref`].
    #[inline]
    #[track_caller]
    pub fn unwrap_mut(self) -> &'a mut T {
        match self {
            Ref(_) => panic!("called `RefKind::unwrap_mut()` on a `RefKind::Ref` value"),
            Mut(unique) => unique,
        }
    }
}

/// Convert immutable reference into [`RefKind`].
impl<'a, T> From<&'a T> for RefKind<'a, T>
where
    T: ?Sized + 'a,
{
    /// Converts to [`Ref`] from the immutable reference.
    #[inline]
    fn from(shared: &'a T) -> Self {
        Ref(shared)
    }
}

/// Convert mutable reference into [`RefKind`].
impl<'a, T> From<&'a mut T> for RefKind<'a, T>
where
    T: ?Sized + 'a,
{
    /// Converts to [`Mut`] from the mutable reference.
    #[inline]
    fn from(unique: &'a mut T) -> Self {
        Mut(unique)
    }
}

impl<'a, T> Deref for RefKind<'a, T>
where
    T: ?Sized + 'a,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Ref(shared) => shared,
            Mut(unique) => unique,
        }
    }
}

/// As suggested in this standard library [section].
///
/// [section]: https://doc.rust-lang.org/std/convert/trait.AsRef.html#generic-implementations
impl<'a, T, R> AsRef<R> for RefKind<'a, T>
where
    T: ?Sized + 'a,
    R: ?Sized,
    <RefKind<'a, T> as Deref>::Target: AsRef<R>,
{
    fn as_ref(&self) -> &R {
        self.deref().as_ref()
    }
}
