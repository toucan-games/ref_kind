/// Provides different kinds of reference:
/// [immutable](RefKind::Ref) or [mutable](RefKind::Mut) one.
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
    /// Returns an immutable reference from the [`RefKind`].
    pub fn get_ref(&self) -> &T {
        match self {
            RefKind::Ref(r#ref) => *r#ref,
            RefKind::Mut(r#mut) => &**r#mut,
        }
    }

    /// Returns [`Some`] with a mutable reference from the struct
    /// or [`None`] if contained reference is immutable.
    pub fn get_mut(&mut self) -> Option<&mut T> {
        match self {
            RefKind::Ref(_) => None,
            RefKind::Mut(r#mut) => Some(*r#mut),
        }
    }

    /// Converts [`RefKind`] into immutable reference
    /// with the lifetime of the owner.
    pub fn into_ref(self) -> &'a T {
        match self {
            RefKind::Ref(r#ref) => r#ref,
            RefKind::Mut(r#mut) => &*r#mut,
        }
    }

    /// Converts [`RefKind`] into optional mutable reference
    /// with the lifetime of the owner.
    ///
    /// Returns [`None`] if contained reference was immutable.
    pub fn into_mut(self) -> Option<&'a mut T> {
        match self {
            RefKind::Ref(_) => None,
            RefKind::Mut(r#mut) => Some(r#mut),
        }
    }
}

/// Convert immutable reference into [`RefKind`].
impl<'a, T> From<&'a T> for RefKind<'a, T>
where
    T: ?Sized + 'a,
{
    // Converts to [`RefKind::Ref`] from the immutable reference.
    fn from(r#ref: &'a T) -> Self {
        Self::Ref(r#ref)
    }
}

/// Convert mutable reference into [`RefKind`].
impl<'a, T> From<&'a mut T> for RefKind<'a, T>
where
    T: ?Sized + 'a,
{
    // Converts to [`RefKind::Mut`] from the mutable reference.
    fn from(r#mut: &'a mut T) -> Self {
        Self::Mut(r#mut)
    }
}
