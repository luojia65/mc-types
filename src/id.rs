use std::borrow::Cow;

pub type Id<'a> = Cow<'a, str>;

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    fn uses_id(_: impl Into<Id<'a>>) {}

    #[test]
    fn test_id() {
        let a = Id::from(Cow::from("minecraft:stone"));
        let b = Id::from(Cow::from(String::from("minecraft:stone")));
        let c = "minecraft:stone";
        let d = String::from("minecraft:stone");
        assert_eq!(a, b);
        uses_id(a);
        uses_id(b);
        uses_id(c);
        uses_id(d);
    }
}
