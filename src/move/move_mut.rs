use crate::{Mut, Ref, RefKind};

use super::{MoveError, Result};

/// Trait for containers which hold *mutable* kind of reference.
///
/// This trait provides method for retrieving a mutable reference
/// by moving it out of the container to preserve the lifetime of the owner.
///
/// This is useful when it is needed to get **many** mutable references
/// on different elements of the owner collection.
///
/// See [crate documentation](crate) for details.
pub trait MoveMut<'owner> {
    /// Type of a mutable reference which is being moved out.
    type Mut: 'owner;

    /// Tries to move a mutable reference out of the container.
    fn move_mut(&mut self) -> Result<Self::Mut>;
}

/// Mutable reference should be moved out of the [`Option`].
impl<'owner, T> MoveMut<'owner> for Option<&'owner mut T>
where
    T: ?Sized,
{
    type Mut = &'owner mut T;

    fn move_mut(&mut self) -> Result<Self::Mut> {
        let unique = self.take().ok_or(MoveError::BorrowedMutably)?;
        Ok(unique)
    }
}

/// Mutable reference should be moved out of the optional [`RefKind`]
/// if the kind of reference is mutable.
impl<'owner, T> MoveMut<'owner> for Option<RefKind<'owner, T>>
where
    T: ?Sized,
{
    type Mut = &'owner mut T;

    fn move_mut(&mut self) -> Result<Self::Mut> {
        let kind = self.take().ok_or(MoveError::BorrowedMutably)?;

        let unique = match kind {
            Ref(shared) => {
                *self = Some(Ref(shared));
                return Err(MoveError::BorrowedImmutably);
            }
            Mut(unique) => unique,
        };
        Ok(unique)
    }
}
