use crate::enchantment::enchantment_kind::EnchantmentKindId;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug)]
pub struct ItemKindId(pub String);

pub trait ItemKind: Eq + Clone + Debug {
    fn id(&self) -> &ItemKindId;
    fn name(&self) -> &str;
    fn compatible_enchantments(&self) -> impl Iterator<Item = &EnchantmentKindId>;

    fn supports(&self, enchantment_kind_id: &EnchantmentKindId) -> bool {
        self.compatible_enchantments()
            .any(|id| id == enchantment_kind_id)
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct OwnedItemKind {
    pub id: ItemKindId,
    pub name: String,
    pub compatible_enchantments: HashSet<EnchantmentKindId>,
}

impl OwnedItemKind {
    fn new(
        id: impl Into<ItemKindId>,
        name: impl Into<String>,
        compatible_enchantments: HashSet<EnchantmentKindId>,
    ) -> Self {
        OwnedItemKind {
            id: id.into(),
            name: name.into(),
            compatible_enchantments,
        }
    }
}

impl ItemKind for OwnedItemKind {
    fn id(&self) -> &ItemKindId {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn compatible_enchantments(&self) -> impl Iterator<Item = &EnchantmentKindId> {
        self.compatible_enchantments.iter()
    }
}
