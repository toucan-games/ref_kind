use crate::{
    kind::RefKind::{self, Mut, Ref},
    many::{Many, MoveError, Result},
};

impl<'a, T, K> Many<'a, K> for Option<RefKind<'a, T>>
where
    T: ?Sized + 'a,
{
    type Ref = &'a T;

    fn try_move_ref(&mut self, _: K) -> Result<Self::Ref> {
        let kind = self.take().ok_or(MoveError::BorrowedMutably)?;

        let shared = kind.into_ref();
        *self = Some(Ref(shared));
        Ok(shared)
    }

    type Mut = &'a mut T;

    fn try_move_mut(&mut self, _: K) -> Result<Self::Mut> {
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
