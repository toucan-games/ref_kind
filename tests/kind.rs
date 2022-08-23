use ref_kind::RefKind;

#[test]
fn from_ref() {
    let number = 42;
    let number_ref = RefKind::from(&number);

    assert!(number_ref.is_ref());
    assert_eq!(RefKind::Ref(&42), number_ref);
}

#[test]
fn from_mut() {
    let mut number = 42;
    let number_mut = RefKind::from(&mut number);

    assert!(number_mut.is_mut());
    assert_eq!(RefKind::Mut(&mut 42), number_mut);
}
