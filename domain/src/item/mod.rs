use crate::enchantment::Enchantment;
use crate::enchantment::enchantment_kind::EnchantmentKindId;
use crate::item::item_kind::ItemKindId;
use serde::{Deserialize, Serialize};

pub mod enchanter;
pub mod item_enchantment_compatibility;
pub mod item_kind;
pub mod item_kind_provider;
pub mod shared_item_enchantment_compatibility_matrix;
pub mod shared_item_kind;
pub mod shared_item_kind_provider;

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Item {
    pub kind: ItemKindId,
    pub enchantments: Vec<Enchantment>,
    pub anvil_use_count: u8,
}

impl Item {
    pub fn new(kind: impl Into<ItemKindId>) -> Self {
        Self::with(kind, vec![], 0)
    }

    pub fn with(
        kind: impl Into<ItemKindId>,
        enchantments: Vec<Enchantment>,
        anvil_use_count: u8,
    ) -> Self {
        Item {
            kind: kind.into(),
            enchantments,
            anvil_use_count,
        }
    }

    pub fn enchantment_kinds(&self) -> impl Iterator<Item = EnchantmentKindId> {
        self.enchantments.iter().map(|enchantment| enchantment.kind)
    }
}
