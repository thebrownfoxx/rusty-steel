use std::fmt::Debug;
use std::hash::Hash;
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash, Debug, Serialize, Deserialize)]
pub struct ItemKindId(pub i8);

impl From<i8> for ItemKindId {
    fn from(value: i8) -> Self {
        ItemKindId(value)
    }
}

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
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
