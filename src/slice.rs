use crate::{Many, Result};

/// Implementation of [`Many`] trait for [slice](prim@slice).
impl<'a, T> Many<'a, usize> for [T]
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
