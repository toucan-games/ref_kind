//! Utilities enabled by `bumpalo` feature.

use core::hash::{BuildHasher, Hash};

use bumpalo::Bump;
use hashbrown::hash_map::{DefaultHashBuilder, Entry};
use hashbrown::{BumpWrapper, HashMap};

use crate::kind::RefKind;

/// Hash map for different kinds of reference which uses [bump allocation](Bump).
///
/// This type provides methods for retrieving references (either immutable or mutable)
/// by moving them out of the map to preserve specified lifetime of the owner.
///
/// Bump allocation is useful to preserve heap space when creating new map.
/// Just drop this instance of map, reset the bump and create new instance while reusing heap space.
#[cfg_attr(docsrs, doc(cfg(feature = "bumpalo")))]
#[repr(transparent)]
pub struct BumpRefKindMap<'a, 'bump, K, V, S = DefaultHashBuilder>
where
    V: ?Sized + 'a,
{
    map: HashMap<K, Option<RefKind<'a, V>>, S, BumpWrapper<'bump>>,
}

impl<'a, 'bump, K, V> BumpRefKindMap<'a, 'bump, K, V, DefaultHashBuilder>
where
    V: ?Sized + 'a,
{
    /// Creates an empty map with provided bump allocator.
    pub fn new(bump: &'bump Bump) -> Self {
        let map = HashMap::new_in(BumpWrapper(bump));
        Self { map }
    }

    /// Creates an empty map with provided bump allocator
    /// with the specified capacity.
    pub fn with_capacity(bump: &'bump Bump, capacity: usize) -> Self {
        let map = HashMap::with_capacity_in(capacity, BumpWrapper(bump));
        Self { map }
    }
}

impl<'a, 'bump, K, V, S> BumpRefKindMap<'a, 'bump, K, V, S>
where
    V: ?Sized + 'a,
{
    /// Creates an empty map with provided bump allocator
    ///  which will use the given hash builder to hash keys.
    pub fn with_hasher(bump: &'bump Bump, hash_builder: S) -> Self {
        let map = HashMap::with_hasher_in(hash_builder, BumpWrapper(bump));
        Self { map }
    }

    /// Creates an empty map with provided bump allocator
    /// and with the specified capacity, using hash_builder to hash the keys.
    pub fn with_capacity_and_hasher(bump: &'bump Bump, capacity: usize, hash_builder: S) -> Self {
        let map = HashMap::with_capacity_and_hasher_in(capacity, hash_builder, BumpWrapper(bump));
        Self { map }
    }
}

impl<'a, 'bump, K, V, S> BumpRefKindMap<'a, 'bump, K, V, S>
where
    K: Eq + Hash,
    V: ?Sized + 'a,
    S: BuildHasher,
{
    /// Returns an immutable reference of the value without preserving lifetime of the owner.
    ///
    /// # Panics
    ///
    /// Panics if mutable reference of the value was already moved out of the map.
    pub fn get_ref(&self, key: &K) -> Option<&V> {
        let option = self.map.get(key)?.as_ref();
        let ref_kind = option.expect(BORROWED_MUTABLY);
        let r#ref = ref_kind.get_ref();
        Some(r#ref)
    }

    /// Returns a mutable reference of the value without preserving lifetime of the owner.
    ///
    /// # Panics
    ///
    /// Panics if mutable reference of the value was already moved out of the map
    /// or the value was already borrowed as immutable.
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let option = self.map.get_mut(key)?.as_mut();
        let ref_kind = option.expect(BORROWED_MUTABLY);
        let r#mut = ref_kind.get_mut().expect(BORROWED_IMMUTABLY);
        Some(r#mut)
    }

    /// Moves an immutable reference of the value out of this map.
    ///
    /// This function copies an immutable reference or replaces mutable reference with immutable one,
    /// preserving an immutable reference in this map.
    ///
    /// # Panics
    ///
    /// Panics if mutable reference of the value was already moved out of the map.
    pub fn move_ref(&mut self, key: K) -> Option<&'a V> {
        match self.map.entry(key) {
            Entry::Occupied(mut occupied) => {
                let ref_kind = occupied.get_mut().as_mut().expect(BORROWED_MUTABLY);
                match ref_kind {
                    RefKind::Ref(r#ref) => Some(*r#ref),
                    RefKind::Mut(_) => {
                        let ref_kind = occupied.insert(None).expect(BORROWED_MUTABLY);
                        let r#ref = ref_kind.into_ref();
                        occupied.insert(Some(RefKind::Ref(r#ref)));
                        Some(r#ref)
                    }
                }
            }
            Entry::Vacant(_) => None,
        }
    }

    /// Moves a mutable reference of the value out of this map.
    ///
    /// # Panics
    ///
    /// Panics if mutable reference of the value was already moved out of the map
    /// or the value was already borrowed as immutable.
    pub fn move_mut(&mut self, key: K) -> Option<&'a mut V> {
        match self.map.entry(key) {
            Entry::Occupied(mut occupied) => {
                let ref_kind = occupied.get_mut().as_mut().expect(BORROWED_MUTABLY);
                match ref_kind {
                    RefKind::Ref(_) => borrowed_immutably_error(),
                    RefKind::Mut(_) => {
                        let ref_kind = occupied.insert(None).expect(BORROWED_MUTABLY);
                        let r#mut = ref_kind.into_mut().expect(BORROWED_IMMUTABLY);
                        Some(r#mut)
                    }
                }
            }
            Entry::Vacant(_) => None,
        }
    }
}

impl<'a, 'bump, K, V, S> From<&'bump Bump> for BumpRefKindMap<'a, 'bump, K, V, S>
where
    K: Eq + Hash,
    V: ?Sized + 'a,
    S: Default,
{
    fn from(bump: &'bump Bump) -> Self {
        let map = HashMap::with_hasher_in(S::default(), BumpWrapper(bump));
        Self { map }
    }
}

impl<'a, 'bump, K, V, S> Extend<(K, &'a V)> for BumpRefKindMap<'a, 'bump, K, V, S>
where
    K: Eq + Hash,
    V: ?Sized + 'a,
    S: BuildHasher,
{
    fn extend<T: IntoIterator<Item = (K, &'a V)>>(&mut self, iter: T) {
        let iter = iter.into_iter().map(|(k, v)| (k, Some(RefKind::Ref(v))));
        self.map.extend(iter)
    }
}

impl<'a, 'bump, K, V, S> Extend<(K, &'a mut V)> for BumpRefKindMap<'a, 'bump, K, V, S>
where
    K: Eq + Hash,
    V: ?Sized + 'a,
    S: BuildHasher,
{
    fn extend<T: IntoIterator<Item = (K, &'a mut V)>>(&mut self, iter: T) {
        let iter = iter.into_iter().map(|(k, v)| (k, Some(RefKind::Mut(v))));
        self.map.extend(iter)
    }
}

const BORROWED_IMMUTABLY: &str = "reference was already borrowed immutably";
const BORROWED_MUTABLY: &str = "reference was already borrowed mutably";

#[cold]
#[track_caller]
fn borrowed_immutably_error() -> ! {
    panic!("{}", BORROWED_IMMUTABLY)
}
