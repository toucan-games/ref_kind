use crate::{Ref, RefKind};

use super::{MoveError, Result};

/// Trait for containers which hold *immutable* kind of reference.
///
/// This trait provides method for retrieving an immutable reference
/// by moving it out of the container to preserve the lifetime of the owner.
///
/// This is useful when it is needed to get **many** mutable references
/// on different elements of the owner collection.
///
/// See [crate documentation](crate) for details.
pub trait MoveRef<'owner> {
    /// Type of an immutable reference which is being moved out.
    type Ref: 'owner;

    /// Tries to move an immutable reference out of the container.
    ///
    /// This function can copy an immutable reference or replace mutable reference with immutable one,
    /// preserving an immutable reference in the container.
    fn move_ref(&mut self) -> Result<Self::Ref>;
}

/// Immutable reference can be trivially copied.
impl<'owner, T> MoveRef<'owner> for &'owner T
where
    T: ?Sized,
{
    type Ref = &'owner T;

    fn move_ref(&mut self) -> Result<Self::Ref> {
        Ok(self)
    }
}

/// Optional immutable reference can be trivially copied.
impl<'owner, T> MoveRef<'owner> for Option<&'owner T>
where
    T: ?Sized,
{
    type Ref = &'owner T;

    fn move_ref(&mut self) -> Result<Self::Ref> {
        let shared = self.ok_or(MoveError::BorrowedImmutably)?;
        Ok(shared)
    }
}

/// Mutable reference should be moved out of the [`Option`]
/// and coerced into immutable one.
impl<'owner, T> MoveRef<'owner> for Option<&'owner mut T>
where
    T: ?Sized,
{
    type Ref = &'owner T;

    fn move_ref(&mut self) -> Result<Self::Ref> {
        let unique = self.take().ok_or(MoveError::BorrowedMutably)?;
        Ok(unique)
    }
}

/// To move immutable reference out of the optional [RefKind],
/// it should copy an immutable reference or replace mutable reference with immutable one,
/// preserving an immutable reference in the container.
impl<'owner, T> MoveRef<'owner> for Option<RefKind<'owner, T>>
where
    T: ?Sized,
{
    type Ref = &'owner T;

    fn move_ref(&mut self) -> Result<Self::Ref> {
        let kind = self.take().ok_or(MoveError::BorrowedMutably)?;

        let shared = kind.into_ref();
        *self = Some(Ref(shared));
        Ok(shared)
    }
}
