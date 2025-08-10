use crate::enchantment::enchantment_kind::EnchantmentKindId;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug)]
pub struct ItemKindId(pub String);

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ItemKind {
    pub id: ItemKindId,
    pub name: String,
    pub compatible_enchantments: HashSet<EnchantmentKindId>,
}

impl ItemKind {
    pub fn new(
        id: impl Into<ItemKindId>,
        name: impl Into<String>,
        compatible_enchantments: HashSet<EnchantmentKindId>,
    ) -> Self {
        ItemKind {
            id: id.into(),
            name: name.into(),
            compatible_enchantments,
        }
    }

    pub fn supports(&self, enchantment_kind_id: &EnchantmentKindId) -> bool {
        self.compatible_enchantments
            .iter()
            .any(|id| id == enchantment_kind_id)
    }
}
