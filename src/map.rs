use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::hash::{BuildHasher, Hash};

use crate::kind::RefKind;

/// Hash map for different kinds of reference.
///
/// This type provides methods for retrieving references (either immutable or mutable)
/// by moving them out of the map to preserve specified lifetime of the owner.
#[repr(transparent)]
pub struct RefKindMap<'data, K, V, S = RandomState>
where
    V: ?Sized,
{
    map: HashMap<K, Option<RefKind<'data, V>>, S>,
}

impl<'data, K, V, S> RefKindMap<'data, K, V, S>
where
    K: Eq + Hash,
    V: ?Sized,
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
        let r#ref = match ref_kind {
            RefKind::Ref(r#ref) => *r#ref,
            RefKind::Mut(r#ref) => &**r#ref,
        };
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
        let r#mut = match ref_kind {
            RefKind::Ref(_) => borrowed_immutably_error(),
            RefKind::Mut(r#mut) => &mut **r#mut,
        };
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
    pub fn move_ref(&mut self, key: K) -> Option<&'data V> {
        let option = self.map.get(&key)?.as_ref();
        let ref_kind = option.expect(BORROWED_MUTABLY);
        let r#ref = match ref_kind {
            RefKind::Ref(r#ref) => *r#ref,
            RefKind::Mut(_) => {
                let option = self.map.remove(&key)?;
                let ref_kind = option.expect(BORROWED_MUTABLY);
                match ref_kind {
                    RefKind::Ref(_) => unreachable!(),
                    RefKind::Mut(r#mut) => {
                        let r#ref = &*r#mut;
                        let ref_kind = Some(RefKind::Ref(r#ref));
                        self.map.insert(key, ref_kind);
                        r#ref
                    }
                }
            }
        };
        Some(r#ref)
    }

    /// Moves a mutable reference of the value out of this map.
    ///
    /// ## Panics
    ///
    /// Panics if mutable reference of the value was already moved out of the map
    /// or the value was already borrowed as immutable.
    pub fn move_mut(&mut self, key: K) -> Option<&'data mut V> {
        let option = self.map.remove(&key)?;
        let ref_kind = option.expect(BORROWED_MUTABLY);
        let r#mut = match ref_kind {
            RefKind::Ref(r#ref) => {
                let ref_kind = Some(RefKind::Ref(r#ref));
                self.map.insert(key, ref_kind);
                borrowed_immutably_error()
            }
            RefKind::Mut(r#mut) => {
                self.map.insert(key, None);
                r#mut
            }
        };
        Some(r#mut)
    }
}

impl<K, V, S> Default for RefKindMap<'_, K, V, S>
where
    K: Eq + Hash,
    V: ?Sized,
    S: Default,
{
    /// Constructs an empty map, with the [Default] value for the hasher.
    fn default() -> Self {
        let map = HashMap::default();
        Self { map }
    }
}

impl<'data, K, V, S> FromIterator<(K, &'data V)> for RefKindMap<'data, K, V, S>
where
    K: Eq + Hash,
    V: ?Sized,
    S: BuildHasher + Default,
{
    fn from_iter<T: IntoIterator<Item = (K, &'data V)>>(iter: T) -> Self {
        let map = iter
            .into_iter()
            .map(|(k, v)| (k, Some(RefKind::Ref(v))))
            .collect();
        Self { map }
    }
}

impl<'data, K, V, S> FromIterator<(K, &'data mut V)> for RefKindMap<'data, K, V, S>
where
    K: Eq + Hash,
    V: ?Sized,
    S: BuildHasher + Default,
{
    fn from_iter<T: IntoIterator<Item = (K, &'data mut V)>>(iter: T) -> Self {
        let map = iter
            .into_iter()
            .map(|(k, v)| (k, Some(RefKind::Mut(v))))
            .collect();
        Self { map }
    }
}

impl<'data, K, V, S> Extend<(K, &'data V)> for RefKindMap<'data, K, V, S>
where
    K: Eq + Hash,
    V: ?Sized,
    S: BuildHasher,
{
    fn extend<T: IntoIterator<Item = (K, &'data V)>>(&mut self, iter: T) {
        let iter = iter.into_iter().map(|(k, v)| (k, Some(RefKind::Ref(v))));
        self.map.extend(iter)
    }
}

impl<'data, K, V, S> Extend<(K, &'data mut V)> for RefKindMap<'data, K, V, S>
where
    K: Eq + Hash,
    V: ?Sized,
    S: BuildHasher,
{
    fn extend<T: IntoIterator<Item = (K, &'data mut V)>>(&mut self, iter: T) {
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
