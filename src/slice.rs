use crate::kind::RefKind;
use crate::many::{Many, MoveError, Result};

/// Implementation of [`Many`] trait for slice of `Option<RefKind<'a, T>>` elements.
impl<'a, T> Many<'a> for [Option<RefKind<'a, T>>]
where
    T: ?Sized + 'a,
{
    type Item = T;

    type Key = usize;

    fn try_move_ref(&mut self, key: Self::Key) -> Result<Option<&'a Self::Item>> {
        let elem = match self.get_mut(key) {
            Some(elem) => elem,
            None => return Ok(None),
        };
        let ref_kind = elem.take().ok_or(MoveError::BorrowedMutably)?;

        let r#ref = ref_kind.into_ref();
        *elem = Some(RefKind::Ref(r#ref));
        Ok(Some(r#ref))
    }

    fn try_move_mut(&mut self, key: Self::Key) -> Result<Option<&'a mut Self::Item>> {
        let elem = match self.get_mut(key) {
            Some(elem) => elem,
            None => return Ok(None),
        };
        let ref_kind = elem.take().ok_or(MoveError::BorrowedMutably)?;

        let r#mut = match ref_kind {
            RefKind::Ref(r#ref) => {
                *elem = Some(RefKind::Ref(r#ref));
                return Err(MoveError::BorrowedImmutably);
            }
            RefKind::Mut(r#mut) => r#mut,
        };
        Ok(Some(r#mut))
    }
}
