// an actual string id for blocks
// available for everything except 'transparent' block
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Id {
    inner: String
}

impl<I: ToString> From<I> for Id{
    fn from(src: I) -> Id {
        Id {
            inner: src.to_string()
        }
    }
}

impl<I: AsRef<str>> PartialEq<I> for Id {
    fn eq(&self, other: &I) -> bool {
        other.as_ref() == self.inner
    }
}  

