use core::hash::{BuildHasher, Hash};
use std_crate::collections::HashMap;

use crate::kind::RefKind;
use crate::many::{Many, MoveError, Result};

/// Implementation of [`Many`] trait for [`HashMap`] of `Option<RefKind<'a, T>>` elements.
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl<'a, K, V, S> Many<'a> for HashMap<K, Option<RefKind<'a, V>>, S>
where
    K: Hash + Eq,
    V: ?Sized + 'a,
    S: BuildHasher,
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
