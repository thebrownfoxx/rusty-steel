use std::fmt::Debug;
use std::hash::Hash;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug)]
pub struct ItemKindId(pub String);

impl From<String> for ItemKindId {
    fn from(value: String) -> Self {
        ItemKindId(value)
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ItemKind {
    pub id: ItemKindId,
    pub name: String,
}

impl ItemKind {
    pub fn new(
        id: impl Into<ItemKindId>,
        name: impl Into<String>,
    ) -> Self {
        ItemKind {
            id: id.into(),
            name: name.into(),
        }
    }
}
