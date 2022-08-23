extern crate alloc;

use alloc::collections::VecDeque;
use alloc::collections::BTreeMap;

use crate::kind::RefKind;
use crate::many::{Many, MoveError, Result};

/// Implementation of [`Many`] trait for [`VecDeque`] of `Option<RefKind<'a, T>>` elements.
impl<'a, T> Many<'a> for VecDeque<Option<RefKind<'a, T>>>
where
    T: ?Sized + 'a,
{
    type Key = usize;

    type Item = T;

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

/// Implementation of [`Many`] trait for [`BTreeMap`] of `Option<RefKind<'a, T>>` elements.
impl<'a, K, V> Many<'a> for BTreeMap<K, Option<RefKind<'a, V>>>
where
    K: Ord,
    V: ?Sized + 'a,
{
    type Key = K;

    type Item = V;

    fn try_move_ref(&mut self, key: Self::Key) -> Result<Option<&'a Self::Item>> {
        let elem = match self.get_mut(&key) {
            Some(elem) => elem,
            None => return Ok(None),
        };
        let ref_kind = elem.take().ok_or(MoveError::BorrowedMutably)?;

        let r#ref = ref_kind.into_ref();
        *elem = Some(RefKind::Ref(r#ref));
        Ok(Some(r#ref))
    }

    fn try_move_mut(&mut self, key: Self::Key) -> Result<Option<&'a mut Self::Item>> {
        let elem = match self.get_mut(&key) {
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