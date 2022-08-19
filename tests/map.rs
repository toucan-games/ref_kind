use ref_kind::RefKindMap;
use std::collections::HashMap;

#[test]
fn from_hash_map() {
    let mut map = HashMap::new();
    map.insert("Hello, World", 0);
    map.insert("Answer", 42);

    let mut map = map
        .iter_mut()
        .map(|(&k, v)| (k, v))
        .collect::<RefKindMap<_, _>>();
    let hello = map.move_mut("Hello, World").unwrap();
    let answer = map.move_mut("Answer").unwrap();
    assert_eq!(*hello, 0);
    assert_eq!(*answer, 42);
}

#[test]
fn multiple_ref() {
    let number = 0;
    let mut map = [("Hello, World", &number)]
        .into_iter()
        .collect::<RefKindMap<_, _>>();

    let first = map.move_ref("Hello, World").unwrap();
    let second = map.move_ref("Hello, World").unwrap();
    assert_eq!(first, second);
}

#[test]
#[should_panic]
fn multiple_mut() {
    let mut number = 0;
    let mut map = [("Hello, World", &mut number)]
        .into_iter()
        .collect::<RefKindMap<_, _>>();

    let first = map.move_ref("Hello, World").unwrap();
    let second = map.move_mut("Hello, World").unwrap();
    assert_eq!(first, second);
}
