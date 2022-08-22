/// Trait for collections which hold different kinds of reference.
///
/// This type provides methods for retrieving references (either immutable or mutable)
/// by moving them out of the collection to preserve the lifetime of the owner.
/// This is useful when it is needed to get **many** mutable references
/// on different elements of the owner collection.
/// See [crate documentation](crate) for a detained explanation.
///
/// This trait is usually implemented for collections of `Option<RefKind<'a, T>>` elements
/// which allows for the implementation to replace [`Some`] with [`None`] when moving out of the collection.
pub trait Many<'a> {
    /// The type of element identifier of the collection.
    type Key;

    /// The type of the elements references of which being moved out.
    type Item: ?Sized + 'a;

    /// Moves an immutable reference out of this collection.
    ///
    /// This function copies an immutable reference or replaces mutable reference with immutable one,
    /// preserving an immutable reference in this collection.
    ///
    /// # Panics
    ///
    /// Panics if mutable reference was already moved out of the collection.
    fn move_ref(&mut self, key: Self::Key) -> Option<&'a Self::Item>;

    /// Moves a mutable reference out of this collection.
    ///
    /// # Panics
    ///
    /// Panics if mutable reference was already moved out of the collection
    /// or the value was already borrowed as immutable.
    fn move_mut(&mut self, key: Self::Key) -> Option<&'a mut Self::Item>;
}
