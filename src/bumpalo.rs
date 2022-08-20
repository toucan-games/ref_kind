//! Utilities enabled by `bumpalo` feature.

use core::borrow::Borrow;
use core::fmt::{self, Debug};
use core::hash::{BuildHasher, Hash};

use bumpalo::Bump;
use hashbrown::hash_map::{
    DefaultHashBuilder, Drain, DrainFilter, Entry, EntryRef, IntoKeys, IntoValues, Iter, IterMut,
    Keys, OccupiedError, Values, ValuesMut,
};
use hashbrown::{BumpWrapper, HashMap, TryReserveError};

use crate::kind::RefKind;

/// Hash map for different kinds of reference which uses [bump allocation](Bump).
///
/// This type provides methods for retrieving references (either immutable or mutable)
/// by moving them out of the map to preserve specified lifetime of the owner.
///
/// Bump allocation is useful to preserve heap space when creating new map.
/// Just drop this instance of map, reset the bump and create new instance while reusing heap space.
///
/// This is needed to satisfy borrow checker:
/// it will not allow for the user to insert new reference into the map from the owner.
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

    /// Returns a reference to the map's [`BuildHasher`].
    ///
    /// [`BuildHasher`]: https://doc.rust-lang.org/std/hash/trait.BuildHasher.html
    pub fn hasher(&self) -> &S {
        self.map.hasher()
    }

    /// Returns the number of elements the map can hold without reallocating.
    ///
    /// This number is a lower bound; the map might be able to hold
    /// more, but is guaranteed to be able to hold at least this many.
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn capacity(&self) -> usize {
        self.map.capacity()
    }

    /// An iterator visiting all keys in arbitrary order.
    /// The iterator element type is `&K`.
    pub fn keys(&self) -> Keys<K, Option<RefKind<'a, V>>> {
        self.map.keys()
    }

    /// An iterator visiting all values in arbitrary order.
    /// The iterator element type is `&Option<RefKind<'a, V>>`.
    pub fn values(&self) -> Values<K, Option<RefKind<'a, V>>> {
        self.map.values()
    }

    /// An iterator visiting all values mutably in arbitrary order.
    /// The iterator element type is `&mut Option<RefKind<'a, V>>`.
    pub fn values_mut(&mut self) -> ValuesMut<K, Option<RefKind<'a, V>>> {
        self.map.values_mut()
    }

    /// An iterator visiting all key-value pairs in arbitrary order.
    /// The iterator element type is `(&K, &Option<RefKind<'a, V>>)`.
    pub fn iter(&self) -> Iter<K, Option<RefKind<'a, V>>> {
        self.map.iter()
    }

    /// An iterator visiting all key-value pairs in arbitrary order,
    /// with mutable references to the values.
    /// The iterator element type is `(&K, &mut Option<RefKind<'a, V>>)`.
    pub fn iter_mut(&mut self) -> IterMut<K, Option<RefKind<'a, V>>> {
        self.map.iter_mut()
    }

    /// Returns the number of elements in the map.
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Returns `true` if the map contains no elements.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Clears the map, returning all key-value pairs as an iterator. Keeps the
    /// allocated memory for reuse.
    ///
    /// If the returned iterator is dropped before being fully consumed, it
    /// drops the remaining key-value pairs. The returned iterator keeps a
    /// mutable borrow on the vector to optimize its implementation.
    pub fn drain(&mut self) -> Drain<K, Option<RefKind<'a, V>>, BumpWrapper<'bump>> {
        self.map.drain()
    }

    /// Retains only the elements specified by the predicate. Keeps the
    /// allocated memory for reuse.
    ///
    /// In other words, remove all pairs `(k, v)` such that `f(&k, &mut v)` returns `false`.
    /// The elements are visited in unsorted (and unspecified) order.
    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&K, &mut Option<RefKind<'a, V>>) -> bool,
    {
        self.map.retain(f)
    }

    /// Drains elements which are true under the given predicate,
    /// and returns an iterator over the removed items.
    ///
    /// In other words, move all pairs `(k, v)` such that `f(&k, &mut v)` returns `true` out
    /// into another iterator.
    ///
    /// Note that `drain_filter` lets you mutate every value in the filter closure, regardless of
    /// whether you choose to keep or remove it.
    ///
    /// When the returned DrainedFilter is dropped, any remaining elements that satisfy
    /// the predicate are dropped from the table.
    ///
    /// It is unspecified how many more elements will be subjected to the closure
    /// if a panic occurs in the closure, or a panic occurs while dropping an element,
    /// or if the `DrainFilter` value is leaked.
    ///
    /// Keeps the allocated memory for reuse.
    pub fn drain_filter<F>(
        &mut self,
        f: F,
    ) -> DrainFilter<K, Option<RefKind<'a, V>>, F, BumpWrapper<'bump>>
    where
        F: FnMut(&K, &mut Option<RefKind<'a, V>>) -> bool,
    {
        self.map.drain_filter(f)
    }

    /// Clears the map, removing all key-value pairs. Keeps the allocated memory
    /// for reuse.
    pub fn clear(&mut self) {
        self.map.clear()
    }

    /// Creates a consuming iterator visiting all the keys in arbitrary order.
    /// The map cannot be used after calling this.
    /// The iterator element type is `K`.
    pub fn into_keys(self) -> IntoKeys<K, Option<RefKind<'a, V>>, BumpWrapper<'bump>> {
        self.map.into_keys()
    }

    /// Creates a consuming iterator visiting all the values in arbitrary order.
    /// The map cannot be used after calling this.
    /// The iterator element type is `Option<RefKind<'a, V>>`.
    pub fn into_values(self) -> IntoValues<K, Option<RefKind<'a, V>>, BumpWrapper<'bump>> {
        self.map.into_values()
    }
}

impl<'a, 'bump, K, V, S> BumpRefKindMap<'a, 'bump, K, V, S>
where
    K: Eq + Hash,
    V: ?Sized + 'a,
    S: BuildHasher,
{
    /// Reserves capacity for at least `additional` more elements to be inserted
    /// in the map. The collection may reserve more space to avoid
    /// frequent reallocations.
    ///
    /// # Panics
    ///
    /// Panics if the new allocation size overflows [`usize`].
    ///
    /// [`usize`]: https://doc.rust-lang.org/std/primitive.usize.html
    pub fn reserve(&mut self, additional: usize) {
        self.map.reserve(additional)
    }

    /// Tries to reserve capacity for at least `additional` more elements to be inserted
    /// in the given map. The collection may reserve more space to avoid
    /// frequent reallocations.
    ///
    /// # Errors
    ///
    /// If the capacity overflows, or the allocator reports a failure, then an error
    /// is returned.
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.map.try_reserve(additional)
    }

    /// Shrinks the capacity of the map as much as possible. It will drop
    /// down as much as possible while maintaining the internal rules
    /// and possibly leaving some space in accordance with the resize policy.
    pub fn shrink_to_fit(&mut self) {
        self.map.shrink_to_fit()
    }

    /// Shrinks the capacity of the map with a lower limit. It will drop
    /// down no lower than the supplied limit while maintaining the internal rules
    /// and possibly leaving some space in accordance with the resize policy.
    ///
    /// This function does nothing if the current capacity is smaller than the
    /// supplied minimum capacity.
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.map.shrink_to(min_capacity)
    }

    /// Gets the given key's corresponding entry in the map for in-place manipulation.
    pub fn entry(&mut self, key: K) -> Entry<K, Option<RefKind<'a, V>>, S, BumpWrapper<'bump>> {
        self.map.entry(key)
    }

    /// Gets the given key's corresponding entry by reference in the map for in-place manipulation.
    pub fn entry_ref<'b, Q>(
        &mut self,
        key: &'b Q,
    ) -> EntryRef<'_, 'b, K, Q, Option<RefKind<'a, V>>, S, BumpWrapper<'bump>>
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        self.map.entry_ref(key)
    }

    /// Returns `true` if the map contains a value for the specified key.
    pub fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.map.contains_key(key)
    }

    /// Returns a reference to the value corresponding to the key.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the key type.
    ///
    /// [`Eq`]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
    /// [`Hash`]: https://doc.rust-lang.org/std/hash/trait.Hash.html
    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&Option<RefKind<'a, V>>>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.map.get(key)
    }

    /// Returns the key-value pair corresponding to the supplied key.
    ///
    /// The supplied key may be any borrowed form of the map's key type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the key type.
    ///
    /// [`Eq`]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
    /// [`Hash`]: https://doc.rust-lang.org/std/hash/trait.Hash.html
    pub fn get_key_value<Q: ?Sized>(&self, key: &Q) -> Option<(&K, &Option<RefKind<'a, V>>)>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.map.get_key_value(key)
    }

    /// Returns an immutable reference of the value without preserving lifetime of the owner.
    ///
    /// The supplied key may be any borrowed form of the map's key type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the key type.
    ///
    /// # Panics
    ///
    /// Panics if mutable reference of the value was already moved out of the map.
    ///
    /// [`Eq`]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
    /// [`Hash`]: https://doc.rust-lang.org/std/hash/trait.Hash.html
    pub fn get_ref(&self, key: &K) -> Option<&V> {
        let option = self.get(key)?.as_ref();
        let ref_kind = option.expect(BORROWED_MUTABLY);
        let r#ref = ref_kind.get_ref();
        Some(r#ref)
    }

    /// Returns key and an immutable reference of the value without preserving lifetime of the owner.
    ///
    /// The supplied key may be any borrowed form of the map's key type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the key type.
    ///
    /// # Panics
    ///
    /// Panics if mutable reference of the value was already moved out of the map.
    ///
    /// [`Eq`]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
    /// [`Hash`]: https://doc.rust-lang.org/std/hash/trait.Hash.html
    pub fn get_key_ref<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        self.get_key_value(key).map(|(key, value)| {
            let value = value.as_ref().expect(BORROWED_MUTABLY).get_ref();
            (key, value)
        })
    }

    /// Returns a mutable reference to the value corresponding to the key.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the key type.
    ///
    /// [`Eq`]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
    /// [`Hash`]: https://doc.rust-lang.org/std/hash/trait.Hash.html
    pub fn get_mut<Q: ?Sized>(&mut self, key: &Q) -> Option<&mut Option<RefKind<'a, V>>>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.map.get_mut(key)
    }

    /// Returns the key-value pair corresponding to the supplied key, with a mutable reference to value.
    ///
    /// The supplied key may be any borrowed form of the map's key type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the key type.
    ///
    /// [`Eq`]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
    /// [`Hash`]: https://doc.rust-lang.org/std/hash/trait.Hash.html
    pub fn get_key_value_mut<Q: ?Sized>(
        &mut self,
        key: &Q,
    ) -> Option<(&K, &mut Option<RefKind<'a, V>>)>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.map.get_key_value_mut(key)
    }

    /// Returns a mutable reference of the value without preserving lifetime of the owner.
    ///
    /// The supplied key may be any borrowed form of the map's key type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the key type.
    ///
    /// # Panics
    ///
    /// Panics if mutable reference of the value was already moved out of the map
    /// or the value was already borrowed as immutable.
    ///
    /// [`Eq`]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
    /// [`Hash`]: https://doc.rust-lang.org/std/hash/trait.Hash.html
    pub fn get_ref_mut(&mut self, key: &K) -> Option<&mut V> {
        let option = self.get_mut(key)?.as_mut();
        let ref_kind = option.expect(BORROWED_MUTABLY);
        let r#mut = ref_kind.get_mut().expect(BORROWED_IMMUTABLY);
        Some(r#mut)
    }

    /// Returns key and a mutable reference of the value without preserving lifetime of the owner.
    ///
    /// The supplied key may be any borrowed form of the map's key type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the key type.
    ///
    /// # Panics
    ///
    /// Panics if mutable reference of the value was already moved out of the map
    /// or the value was already borrowed as immutable.
    ///
    /// [`Eq`]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
    /// [`Hash`]: https://doc.rust-lang.org/std/hash/trait.Hash.html
    pub fn get_key_ref_mut<Q: ?Sized>(&mut self, key: &Q) -> Option<(&K, &mut V)>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.get_key_value_mut(key).map(|(key, value)| {
            let value = value.as_mut().expect(BORROWED_MUTABLY);
            let value = value.get_mut().expect(BORROWED_IMMUTABLY);
            (key, value)
        })
    }

    /// Inserts a key and an immutable reference pair into the map.
    ///
    /// If the map did not have this key present, [`None`] is returned.
    ///
    /// If the map did have this key present, the value is updated, and the old
    /// value is returned. The key is not updated, though; this matters for
    /// types that can be `==` without being identical.
    pub fn insert_ref(&mut self, key: K, value: &'a V) -> Option<RefKind<'a, V>> {
        let value = Some(RefKind::Ref(value));
        self.map.insert(key, value).flatten()
    }

    /// Inserts a key and a mutable reference pair into the map.
    ///
    /// If the map did not have this key present, [`None`] is returned.
    ///
    /// If the map did have this key present, the value is updated, and the old
    /// value is returned. The key is not updated, though; this matters for
    /// types that can be `==` without being identical.
    pub fn insert_ref_mut(&mut self, key: K, value: &'a mut V) -> Option<RefKind<'a, V>> {
        let value = Some(RefKind::Mut(value));
        self.map.insert(key, value).flatten()
    }

    /// Tries to insert a key and an immutable reference pair into the map, and returns
    /// a mutable reference to the value in the entry.
    ///
    /// # Errors
    ///
    /// If the map already had this key present, nothing is updated, and
    /// an error containing the occupied entry and the value is returned.
    #[allow(clippy::type_complexity)]
    pub fn try_insert_ref(
        &mut self,
        key: K,
        value: &'a V,
    ) -> Result<
        &mut Option<RefKind<'a, V>>,
        OccupiedError<K, Option<RefKind<'a, V>>, S, BumpWrapper<'bump>>,
    > {
        let value = Some(RefKind::Ref(value));
        self.map.try_insert(key, value)
    }

    /// Tries to insert a key and a mutable reference pair into the map, and returns
    /// a mutable reference to the value in the entry.
    ///
    /// # Errors
    ///
    /// If the map already had this key present, nothing is updated, and
    /// an error containing the occupied entry and the value is returned.
    #[allow(clippy::type_complexity)]
    pub fn try_insert_ref_mut(
        &mut self,
        key: K,
        value: &'a mut V,
    ) -> Result<
        &mut Option<RefKind<'a, V>>,
        OccupiedError<K, Option<RefKind<'a, V>>, S, BumpWrapper<'bump>>,
    > {
        let value = Some(RefKind::Mut(value));
        self.map.try_insert(key, value)
    }

    /// Removes a key from the map, returning the value at the key if the key
    /// was previously in the map. Keeps the allocated memory for reuse.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the key type.
    ///
    /// [`Eq`]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
    /// [`Hash`]: https://doc.rust-lang.org/std/hash/trait.Hash.html
    pub fn remove<Q>(&mut self, key: &Q) -> Option<Option<RefKind<V>>>
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        self.map.remove(key)
    }

    /// Removes a key from the map, returning the stored key and value if the
    /// key was previously in the map. Keeps the allocated memory for reuse.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the key type.
    ///
    /// [`Eq`]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
    /// [`Hash`]: https://doc.rust-lang.org/std/hash/trait.Hash.html
    pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(K, Option<RefKind<V>>)>
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        self.map.remove_entry(key)
    }

    /// Moves an immutable reference of the value out of this map.
    ///
    /// This function copies an immutable reference or replaces mutable reference with immutable one,
    /// preserving an immutable reference in this map.
    ///
    /// The supplied key may be any borrowed form of the map's key type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the key type.
    ///
    /// # Panics
    ///
    /// Panics if mutable reference of the value was already moved out of the map.
    ///
    /// [`Eq`]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
    /// [`Hash`]: https://doc.rust-lang.org/std/hash/trait.Hash.html
    pub fn move_ref<Q>(&mut self, key: &Q) -> Option<&'a V>
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        match self.entry_ref(key) {
            EntryRef::Occupied(mut occupied) => {
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
            EntryRef::Vacant(_) => None,
        }
    }

    /// Moves a mutable reference of the value out of this map.
    ///
    /// The supplied key may be any borrowed form of the map's key type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the key type.
    ///
    /// # Panics
    ///
    /// Panics if mutable reference of the value was already moved out of the map
    /// or the value was already borrowed as immutable.
    ///
    /// [`Eq`]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
    /// [`Hash`]: https://doc.rust-lang.org/std/hash/trait.Hash.html
    pub fn move_mut<Q>(&mut self, key: &Q) -> Option<&'a mut V>
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        match self.entry_ref(key) {
            EntryRef::Occupied(mut occupied) => {
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
            EntryRef::Vacant(_) => None,
        }
    }
}

impl<'a, 'bump, K, V, S> PartialEq for BumpRefKindMap<'a, 'bump, K, V, S>
where
    K: Eq + Hash,
    V: ?Sized + 'a + PartialEq,
    S: BuildHasher,
{
    fn eq(&self, other: &Self) -> bool {
        self.map == other.map
    }
}

impl<'a, 'bump, K, V, S> Eq for BumpRefKindMap<'a, 'bump, K, V, S>
where
    K: Eq + Hash,
    V: ?Sized + 'a + Eq,
    S: BuildHasher,
{
}

impl<'a, 'bump, K, V, S> Debug for BumpRefKindMap<'a, 'bump, K, V, S>
where
    K: Debug,
    V: ?Sized + 'a + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.map.fmt(f)
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
