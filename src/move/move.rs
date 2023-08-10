#![allow(clippy::module_inception)]

use crate::{Many, Result};

use super::{MoveMut, MoveRef};

/// Trait for containers which hold *different* kinds of reference.
///
/// Combines [`MoveRef`] and [`MoveMut`] functionality together,
/// allowing to move immutable and mutable references from the same container.
pub trait Move<'owner>: MoveRef<'owner> + MoveMut<'owner> {}

/// Technically, `Move` is a trait alias to `MoveRef + MoveMut` trait combination.
impl<'owner, T> Move<'owner> for T where T: ?Sized + MoveRef<'owner> + MoveMut<'owner> {}

/// [`Many`] trait can be implemented for any type which implements [`Move`] trait for any key.
impl<'owner, T, K> Many<'owner, K> for T
where
    T: ?Sized + Move<'owner>,
{
    type Ref = <Self as MoveRef<'owner>>::Ref;

    fn try_move_ref(&mut self, _: K) -> Result<Self::Ref> {
        MoveRef::move_ref(self)
    }

    type Mut = <Self as MoveMut<'owner>>::Mut;

    fn try_move_mut(&mut self, _: K) -> Result<Self::Mut> {
        MoveMut::move_mut(self)
    }
}
