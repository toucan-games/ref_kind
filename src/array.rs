use crate::kind::RefKind;
use crate::many::Many;

/// Implementation of [`Many`] trait for slice of `Option<RefKind<'a, T>>`.
impl<'a, T> Many<'a> for [Option<RefKind<'a, T>>]
where
    T: ?Sized + 'a,
{
    type Item = T;

    type Key = usize;

    fn move_ref(&mut self, key: Self::Key) -> Option<&'a Self::Item> {
        let elem = self.get_mut(key)?;
        let ref_kind = elem.take().expect(BORROWED_MUTABLY);

        let r#ref = ref_kind.into_ref();
        *elem = Some(RefKind::Ref(r#ref));
        Some(r#ref)
    }

    fn move_mut(&mut self, key: Self::Key) -> Option<&'a mut Self::Item> {
        let elem = self.get_mut(key)?;
        let ref_kind = elem.take().expect(BORROWED_MUTABLY);

        let r#mut = match ref_kind {
            RefKind::Ref(r#ref) => {
                *elem = Some(RefKind::Ref(r#ref));
                borrowed_immutably_error()
            }
            RefKind::Mut(r#mut) => r#mut,
        };
        Some(r#mut)
    }
}

/// Implementation of [`Many`] trait for array of `Option<RefKind<'a, T>>`.
impl<'a, T, const N: usize> Many<'a> for [Option<RefKind<'a, T>>; N]
where
    T: ?Sized + 'a,
{
    type Item = T;

    type Key = usize;

    fn move_ref(&mut self, key: Self::Key) -> Option<&'a Self::Item> {
        self.as_mut_slice().move_ref(key)
    }

    fn move_mut(&mut self, key: Self::Key) -> Option<&'a mut Self::Item> {
        self.as_mut_slice().move_mut(key)
    }
}

const BORROWED_IMMUTABLY: &str = "reference was already borrowed immutably";
const BORROWED_MUTABLY: &str = "reference was already borrowed mutably";

#[cold]
#[track_caller]
fn borrowed_immutably_error() -> ! {
    panic!("{}", BORROWED_IMMUTABLY)
}
