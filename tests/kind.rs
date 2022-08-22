use ref_kind::RefKind;

#[test]
fn from() {
    let number = 42;
    let number_ref = RefKind::from(&number);

    let is_ref = matches!(number_ref, RefKind::Ref(_));
    assert!(is_ref);
    assert_eq!(RefKind::Ref(&42), number_ref);
}

#[test]
fn from_mut() {
    let mut number = 42;
    let number_mut = RefKind::from(&mut number);

    let is_mut = matches!(number_mut, RefKind::Mut(_));
    assert!(is_mut);
    assert_eq!(RefKind::Mut(&mut 42), number_mut);
}
