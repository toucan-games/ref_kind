/// The result of moving reference out of the value.
pub type Result<T> = core::result::Result<T, MoveError>;

/// Enum that defines errors which can occur when moving reference
/// out of the value.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MoveError {
    /// Reference was already moved out of the collection as immutable.
    /// It is not allowed to get mutable reference again, but it is allowed to get immutable one.
    BorrowedImmutably,
    /// Reference was already moved out of the collection as mutable.
    /// It is not allowed to get neither immutable nor mutable reference again.
    BorrowedMutably,
}

impl core::fmt::Display for MoveError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::BorrowedImmutably => write!(f, "reference was already borrowed immutably"),
            Self::BorrowedMutably => write!(f, "reference was already borrowed mutably"),
        }
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl std_crate::error::Error for MoveError {}
