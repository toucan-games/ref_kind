use core::hash::{BuildHasher, Hash};
use hashbrown::HashMap;

use crate::{Many, Result};

/// Implementation of [`Many`] trait for [`hashbrown::HashMap`].
#[cfg_attr(docsrs, doc(cfg(feature = "hashbrown")))]
impl<'a, K, V, S> Many<'a, K> for HashMap<K, V, S>
where
    K: Hash + Eq,
    V: Many<'a, K>,
    S: BuildHasher,
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
