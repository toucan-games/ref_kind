use std::collections::hash_map::{RandomState, Entry};
use std::collections::HashMap;
use std::hash::{BuildHasher, Hash};

use crate::kind::RefKind;

/// Hash map for different kinds of reference.
///
/// This type provides methods for retrieving references (either immutable or mutable)
/// by moving them out of the map to preserve specified lifetime of the owner.
#[repr(transparent)]
pub struct RefKindMap<'a, K, V, S = RandomState>
where
    V: ?Sized + 'a,
{
    map: HashMap<K, Option<RefKind<'a, V>>, S>,
}

impl<'a, K, V, S> RefKindMap<'a, K, V, S>
where
    K: Eq + Hash,
    V: ?Sized + 'a,
    S: BuildHasher,
{
    /// Returns an immutable reference of the value without preserving lifetime of the owner.
    ///
    /// ## Panics
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
    /// ## Panics
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
    /// ## Panics
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
                    },
                }
            },
            Entry::Vacant(_) => None,
        }
    }

    /// Moves a mutable reference of the value out of this map.
    ///
    /// ## Panics
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
                    },
                }
            },
            Entry::Vacant(_) => None,
        }
    }
}

impl<'a, K, V, S> Default for RefKindMap<'a, K, V, S>
where
    K: Eq + Hash,
    V: ?Sized + 'a,
    S: Default,
{
    /// Constructs an empty map, with the [Default] value for the hasher.
    fn default() -> Self {
        let map = HashMap::default();
        Self { map }
    }
}

impl<'a, K, V, S> FromIterator<(K, &'a V)> for RefKindMap<'a, K, V, S>
where
    K: Eq + Hash,
    V: ?Sized + 'a,
    S: BuildHasher + Default,
{
    fn from_iter<T: IntoIterator<Item = (K, &'a V)>>(iter: T) -> Self {
        let map = iter
            .into_iter()
            .map(|(k, v)| (k, Some(RefKind::Ref(v))))
            .collect();
        Self { map }
    }
}

impl<'a, K, V, S> FromIterator<(K, &'a mut V)> for RefKindMap<'a, K, V, S>
where
    K: Eq + Hash,
    V: ?Sized + 'a,
    S: BuildHasher + Default,
{
    fn from_iter<T: IntoIterator<Item = (K, &'a mut V)>>(iter: T) -> Self {
        let map = iter
            .into_iter()
            .map(|(k, v)| (k, Some(RefKind::Mut(v))))
            .collect();
        Self { map }
    }
}

impl<'a, K, V, S> Extend<(K, &'a V)> for RefKindMap<'a, K, V, S>
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

impl<'a, K, V, S> Extend<(K, &'a mut V)> for RefKindMap<'a, K, V, S>
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
