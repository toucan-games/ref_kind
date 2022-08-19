use ref_kind::RefKind;

#[test]
fn from() {
    let number = 42;
    let number_ref = RefKind::from(&number);
    assert!(matches!(number_ref, RefKind::Ref(_)));
    assert_eq!(RefKind::Ref(&42), number_ref);
}

#[test]
fn from_mut() {
    let mut number = 42;
    let number_mut = RefKind::from(&mut number);
    assert!(matches!(number_mut, RefKind::Mut(_)));
    assert_eq!(RefKind::Mut(&mut 42), number_mut);
}
