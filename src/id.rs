// .0:  None: transparent, Some("air"): air, Some("stone"): etc
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct BlockId(Option<&'static str>);

impl From<Option<&'static str>> for BlockId {
    fn from(src: Option<&'static str>) -> BlockId {
        BlockId(src)
    }
}

impl From<&'static str> for BlockId {
    fn from(src: &'static str) -> BlockId {
        BlockId(Some(src))
    }
}

impl PartialEq<Option<&'static str>> for BlockId {
    fn eq(&self, other: &Option<&'static str>) -> bool {
        self.0.eq(other)
    }
}

impl PartialEq<BlockId> for Option<&'static str> {
    fn eq(&self, other: &BlockId) -> bool {
        other.0.eq(self)
    }
}
