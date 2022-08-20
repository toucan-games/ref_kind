#![cfg(feature = "bumpalo")]

use bumpalo::Bump;
use ref_kind::bumpalo::BumpRefKindMap;

#[test]
fn reuse_heap() {
    // Our bump is ready to rock!
    let mut bump = Bump::new();
    // Create vector of `String`s and integers
    let mut vec = (0..10)
        .map(|int| (int.to_string(), int))
        .collect::<Vec<_>>();

    let old_capacity = {
        // Create our map which will reuse heap space later
        let mut map = BumpRefKindMap::new(&bump);
        map.extend(vec.iter_mut().map(|(str, int)| (str.as_str(), int)));

        let one = map.move_mut("1").unwrap();
        assert_eq!(*one, 1);
        let four = map.move_mut("4").unwrap();
        assert_eq!(*four, 4);

        // Get the capacity after map extending
        bump.chunk_capacity()
    };

    // Reset the bump to reuse heap space
    bump.reset();

    // Create the same map with the same size
    let mut map = BumpRefKindMap::new(&bump);
    map.extend(vec.iter_mut().map(|(str, int)| (str.as_str(), int)));

    // Get the new capacity after creating brand new map
    let new_capacity = bump.chunk_capacity();
    // Both old and new capacities must be equal - no reallocation!
    assert_eq!(old_capacity, new_capacity);
}
