use std::borrow::Cow;

macro_rules! define_id {
    ($($struct_name: ident)+) => {
        $(

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct $struct_name(String);

impl<I: Into<Cow<'a, str>>> From<I> for $struct_name {
    fn from(id: I) -> $struct_name {
        $struct_name(id.into().into_owned())
    }
}

        )+
    };
}

define_id!(BlockId EntityId);

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    fn uses_block_id(_: impl Into<BlockId>) {}

    #[test]
    fn test_block_id() {
        let a = BlockId::from(Cow::from("minecraft:stone"));
        let b = BlockId::from(Cow::from(String::from("minecraft:stone")));
        let c = "minecraft:stone";
        let d = String::from("minecraft:stone");
        assert_eq!(a, b);
        uses_block_id(a);
        uses_block_id(b);
        uses_block_id(c);
        uses_block_id(d);
    }
}
