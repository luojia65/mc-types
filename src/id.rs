// an actual string id for blocks
// available for everything except 'transparent' block
use alloc::borrow::Cow;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Id<'a> {
    inner: Cow<'a, str>
}

impl<'a> Id<'a> {
    pub fn new(raw: impl Into<Cow<'a, str>>) -> Id<'a> {
        Id {
            inner: raw.into()
        }
    }
}

impl<'a, I: AsRef<str>> PartialEq<I> for Id<'a> {
    fn eq(&self, other: &I) -> bool {
        other.as_ref() == self.inner
    }
}  

impl<'a> ToString for Id<'a> {
    fn to_string(&self) -> String {
        self.inner.clone().into_owned()
    }
}

