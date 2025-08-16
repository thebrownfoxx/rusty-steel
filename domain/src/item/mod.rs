use crate::enchantment::Enchantment;
use crate::item::item_kind::ItemKindId;

pub mod item_kind;
pub mod item_kind_provider;
pub mod shared_item_kind;
pub mod shared_item_kind_provider;
mod item_enchantment_compatibility;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Item {
    pub kind: ItemKindId,
    pub enchantments: Vec<Enchantment>,
    pub anvil_use_count: u8,
}

impl Item {
    pub fn new(
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
}
