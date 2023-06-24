//! Provides peekable key for [`Peekable`] iterator
//! and implementation of [`Many`] trait for this type of iterator.

use core::iter::Peekable;

use crate::many::{Many, Result};

/// Type of key for peekable iterator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PeekableKey<Key> {
    /// Pass key to the peeked item.
    Peek(Key),
    /// Pass key to the `n`th item.
    Nth(Key, usize),
}

impl<Key> PeekableKey<Key> {
    /// Creates new peekable key that passes provided key to the peeked element.
    pub fn peek(key: Key) -> Self {
        Self::Peek(key)
    }

    /// Creates new peekable key that passes provided key to the next element.
    pub fn next(key: Key) -> Self {
        Self::Nth(key, 0)
    }

    /// Creates new peekable key that passes provided key to the `n`th element.
    pub fn nth(key: Key, n: usize) -> Self {
        Self::Nth(key, n)
    }

    /// Turns this peekable key into the inner key.
    pub fn into_key(self) -> Key {
        match self {
            Self::Peek(key) => key,
            Self::Nth(key, _) => key,
        }
    }
}

impl<Key> Default for PeekableKey<Key>
where
    Key: Default,
{
    fn default() -> Self {
        let key = Default::default();
        Self::Peek(key)
    }
}

impl<'a, I, Item, Key> Many<'a, PeekableKey<Key>> for Peekable<I>
where
    I: Iterator<Item = Item>,
    Item: Many<'a, Key>,
{
    type Ref = Option<Item::Ref>;

    fn try_move_ref(&mut self, key: PeekableKey<Key>) -> Result<Self::Ref> {
        let (key, item) = peek_by_key(self, key);
        item.map(|item| item.try_move_ref(key)).transpose()
    }

    type Mut = Option<Item::Mut>;

    fn try_move_mut(&mut self, key: PeekableKey<Key>) -> Result<Self::Mut> {
        let (key, item) = peek_by_key(self, key);
        item.map(|item| item.try_move_mut(key)).transpose()
    }
}

fn peek_by_key<I, Key>(iter: &mut Peekable<I>, key: PeekableKey<Key>) -> (Key, Option<&mut I::Item>)
where
    I: Iterator,
{
    match key {
        PeekableKey::Peek(key) => (key, iter.peek_mut()),
        PeekableKey::Nth(key, n) => {
            let _ = iter.nth(n);
            (key, iter.peek_mut())
        }
    }
}
