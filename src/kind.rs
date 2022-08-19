/// Provides different kinds of reference:
/// [immutable](RefKind::Ref) or [mutable](RefKind::Mut) one.
pub enum RefKind<'a, T>
where
    T: ?Sized,
{
    /// Immutable kind of reference.
    Ref(&'a T),
    /// Mutable kind of reference.
    Mut(&'a mut T),
}

/// Convert immutable reference into [`RefKind`].
impl<'a, T> From<&'a T> for RefKind<'a, T>
where
    T: ?Sized,
{
    fn from(r#ref: &'a T) -> Self {
        Self::Ref(r#ref)
    }
}

/// Convert mutable reference into [`RefKind`].
impl<'a, T> From<&'a mut T> for RefKind<'a, T>
where
    T: ?Sized,
{
    fn from(r#mut: &'a mut T) -> Self {
        Self::Mut(r#mut)
    }
}
