use crate::{MoveError, Result};

/// Trait for collections which hold different kinds of reference.
///
/// This trait provides methods for retrieving references (either immutable or mutable)
/// by moving them out of the collection to preserve the lifetime of the owner.
///
/// This is useful when it is needed to get **many** mutable references
/// on different elements of the owner collection.
///
/// See [crate documentation](crate) for details.
pub trait Many<'a, Key> {
    /// The type of a reference which is being moved out.
    type Ref: 'a;

    /// Tries to move an immutable reference out of this collection.
    ///
    /// This function copies an immutable reference or replaces mutable reference with immutable one,
    /// preserving an immutable reference in this collection.
    fn try_move_ref(&mut self, key: Key) -> Result<Self::Ref>;

    /// Moves an immutable reference out of this collection.
    ///
    /// This function copies an immutable reference or replaces mutable reference with immutable one,
    /// preserving an immutable reference in this collection.
    ///
    /// # Panics
    ///
    /// Panics if mutable reference was already moved out of the collection.
    #[track_caller]
    fn move_ref(&mut self, key: Key) -> Self::Ref {
        match self.try_move_ref(key) {
            Ok(result) => result,
            Err(error) => move_panic(error),
        }
    }

    /// The type of a mutable reference which is being moved out.
    type Mut: 'a;

    /// Tries to move a mutable reference out of this collection.
    fn try_move_mut(&mut self, key: Key) -> Result<Self::Mut>;

    /// Moves a mutable reference out of this collection.
    ///
    /// # Panics
    ///
    /// Panics if mutable reference was already moved out of the collection
    /// or the value was already borrowed as immutable.
    #[track_caller]
    fn move_mut(&mut self, key: Key) -> Self::Mut {
        match self.try_move_mut(key) {
            Ok(option) => option,
            Err(error) => move_panic(error),
        }
    }
}

#[cold]
#[track_caller]
fn move_panic(error: MoveError) -> ! {
    panic!("{}", error)
}
