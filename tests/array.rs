use core::array;

use ref_kind::{Many, RefKind};

#[test]
#[should_panic(expected = "reference was already borrowed mutably")]
fn many_array() {
    // Create an array of square of integers from 0 to 9
    let mut array: [_; 10] = array::from_fn(|i| i * i);

    // Create vector of mutable references on all of the array elements
    let mut many = array
        .iter_mut()
        .map(|r#mut| Some(RefKind::Mut(r#mut)))
        .collect::<Vec<_>>();

    // Move out mutable reference by index 1
    // It is no longer in the vector
    let one = many.move_mut(1).unwrap();
    assert_eq!(*one, 1);

    // Move out immutable reference by index 4
    // Vector now contains immutable reference, not mutable one
    let four = many.move_ref(4).unwrap();
    assert_eq!(*four, 16);
    // Move it again: no panic here because immutable reference was copied
    let four_again = many.move_ref(4).unwrap();
    assert_eq!(four, four_again);

    // This call will panic because vector contains no reference by index 1
    let one_again = many.move_ref(1).unwrap();
    assert_eq!(one_again, one);
}
