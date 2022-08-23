use core::fmt::{Display, Formatter};

/// Trait for collections which hold different kinds of reference.
///
/// This trait provides methods for retrieving references (either immutable or mutable)
/// by moving them out of the collection to preserve the lifetime of the owner.
/// This is useful when it is needed to get **many** mutable references
/// on different elements of the owner collection.
/// See [crate documentation](crate) for a detailed explanation and an example.
///
/// This trait is usually implemented for collections of `Option<RefKind<'a, T>>` elements
/// which allows for the implementation to replace [`Some`] with [`None`] when moving out of the collection.
pub trait Many<'a> {
    /// The type of element identifier of the collection.
    type Key;

    /// The type of the elements references of which being moved out.
    type Item: ?Sized + 'a;

    /// Tries to move an immutable reference out of this collection.
    ///
    /// This function copies an immutable reference or replaces mutable reference with immutable one,
    /// preserving an immutable reference in this collection.
    fn try_move_ref(&mut self, key: Self::Key) -> Result<Option<&'a Self::Item>>;

    /// Moves an immutable reference out of this collection.
    ///
    /// This function copies an immutable reference or replaces mutable reference with immutable one,
    /// preserving an immutable reference in this collection.
    ///
    /// # Panics
    ///
    /// Panics if mutable reference was already moved out of the collection.
    fn move_ref(&mut self, key: Self::Key) -> Option<&'a Self::Item> {
        match self.try_move_ref(key) {
            Ok(option) => option,
            Err(error) => move_panic(error),
        }
    }

    /// Tries to move a mutable reference out of this collection.
    fn try_move_mut(&mut self, key: Self::Key) -> Result<Option<&'a mut Self::Item>>;

    /// Moves a mutable reference out of this collection.
    ///
    /// # Panics
    ///
    /// Panics if mutable reference was already moved out of the collection
    /// or the value was already borrowed as immutable.
    fn move_mut(&mut self, key: Self::Key) -> Option<&'a mut Self::Item> {
        match self.try_move_mut(key) {
            Ok(option) => option,
            Err(error) => move_panic(error),
        }
    }
}

/// The result of [`Many`] trait method calls.
pub type Result<T> = core::result::Result<T, MoveError>;

/// Enum that defines errors which can occur when moving reference
/// out of the collection which implements [`Many`] trait.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MoveError {
    /// Reference was already moved out of the collection as immutable.
    /// It is not allowed to get mutable reference again, but it is allowed to get immutable one.
    BorrowedImmutably,
    /// Reference was already moved out of the collection as mutable.
    /// It is not allowed to get neither immutable nor mutable reference again.
    BorrowedMutably,
}

impl Display for MoveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            MoveError::BorrowedImmutably => write!(f, "reference was already borrowed immutably"),
            MoveError::BorrowedMutably => write!(f, "reference was already borrowed mutably"),
        }
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl std_crate::error::Error for MoveError {}

#[cold]
#[track_caller]
fn move_panic(error: MoveError) -> ! {
    panic!("{}", error)
}
