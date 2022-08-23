use alloc_crate::collections::BTreeMap;
use alloc_crate::collections::VecDeque;

use crate::kind::RefKind;
use crate::many::{Many, MoveError, Result};

/// Implementation of [`Many`] trait for [`VecDeque`] of `Option<RefKind<'a, T>>` elements.
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<'a, T> Many<'a> for VecDeque<Option<RefKind<'a, T>>>
where
    T: ?Sized + 'a,
{
    type Key = usize;

    type Item = T;

    fn try_move_ref(&mut self, key: Self::Key) -> Result<Option<&'a Self::Item>> {
        let item = match self.get_mut(key) {
            Some(item) => item,
            None => return Ok(None),
        };
        let ref_kind = item.take().ok_or(MoveError::BorrowedMutably)?;

        let shared = ref_kind.into_ref();
        *item = Some(RefKind::Ref(shared));
        Ok(Some(shared))
    }

    fn try_move_mut(&mut self, key: Self::Key) -> Result<Option<&'a mut Self::Item>> {
        let item = match self.get_mut(key) {
            Some(item) => item,
            None => return Ok(None),
        };
        let ref_kind = item.take().ok_or(MoveError::BorrowedMutably)?;

        let unique = match ref_kind {
            RefKind::Ref(shared) => {
                *item = Some(RefKind::Ref(shared));
                return Err(MoveError::BorrowedImmutably);
            }
            RefKind::Mut(unique) => unique,
        };
        Ok(Some(unique))
    }
}

/// Implementation of [`Many`] trait for [`BTreeMap`] of `Option<RefKind<'a, T>>` elements.
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<'a, K, V> Many<'a> for BTreeMap<K, Option<RefKind<'a, V>>>
where
    K: Ord,
    V: ?Sized + 'a,
{
    type Key = K;

    type Item = V;

    fn try_move_ref(&mut self, key: Self::Key) -> Result<Option<&'a Self::Item>> {
        let item = match self.get_mut(&key) {
            Some(item) => item,
            None => return Ok(None),
        };
        let ref_kind = item.take().ok_or(MoveError::BorrowedMutably)?;

        let shared = ref_kind.into_ref();
        *item = Some(RefKind::Ref(shared));
        Ok(Some(shared))
    }

    fn try_move_mut(&mut self, key: Self::Key) -> Result<Option<&'a mut Self::Item>> {
        let item = match self.get_mut(&key) {
            Some(item) => item,
            None => return Ok(None),
        };
        let ref_kind = item.take().ok_or(MoveError::BorrowedMutably)?;

        let unique = match ref_kind {
            RefKind::Ref(shared) => {
                *item = Some(RefKind::Ref(shared));
                return Err(MoveError::BorrowedImmutably);
            }
            RefKind::Mut(unique) => unique,
        };
        Ok(Some(unique))
    }
}
