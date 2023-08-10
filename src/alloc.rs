use alloc_crate::{
    collections::{BTreeMap, VecDeque},
    vec::Vec,
};

use crate::{Many, Result};

/// Implementation of [`Many`] trait for [`Vec`].
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<'a, T> Many<'a, usize> for Vec<T>
where
    T: Many<'a, usize>,
{
    type Ref = Option<T::Ref>;

    fn try_move_ref(&mut self, key: usize) -> Result<Self::Ref> {
        let item = match self.get_mut(key) {
            Some(item) => item,
            None => return Ok(None),
        };
        let shared = item.try_move_ref(key)?;
        Ok(Some(shared))
    }

    type Mut = Option<T::Mut>;

    fn try_move_mut(&mut self, key: usize) -> Result<Self::Mut> {
        let item = match self.get_mut(key) {
            Some(item) => item,
            None => return Ok(None),
        };
        let unique = item.try_move_mut(key)?;
        Ok(Some(unique))
    }
}

/// Implementation of [`Many`] trait for [`VecDeque`].
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<'a, T> Many<'a, usize> for VecDeque<T>
where
    T: Many<'a, usize>,
{
    type Ref = Option<T::Ref>;

    fn try_move_ref(&mut self, key: usize) -> Result<Self::Ref> {
        let item = match self.get_mut(key) {
            Some(item) => item,
            None => return Ok(None),
        };
        let shared = item.try_move_ref(key)?;
        Ok(Some(shared))
    }

    type Mut = Option<T::Mut>;

    fn try_move_mut(&mut self, key: usize) -> Result<Self::Mut> {
        let item = match self.get_mut(key) {
            Some(item) => item,
            None => return Ok(None),
        };
        let unique = item.try_move_mut(key)?;
        Ok(Some(unique))
    }
}

/// Implementation of [`Many`] trait for [`BTreeMap`].
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<'a, K, V> Many<'a, K> for BTreeMap<K, V>
where
    K: Ord,
    V: Many<'a, K>,
{
    type Ref = Option<V::Ref>;

    fn try_move_ref(&mut self, key: K) -> Result<Self::Ref> {
        let item = match self.get_mut(&key) {
            Some(item) => item,
            None => return Ok(None),
        };
        let shared = item.try_move_ref(key)?;
        Ok(Some(shared))
    }

    type Mut = Option<V::Mut>;

    fn try_move_mut(&mut self, key: K) -> Result<Self::Mut> {
        let item = match self.get_mut(&key) {
            Some(item) => item,
            None => return Ok(None),
        };
        let unique = item.try_move_mut(key)?;
        Ok(Some(unique))
    }
}
