use crate::enchantment::enchantment::Enchantment;
use crate::item::item_type::ItemType;

pub struct Item {
    pub item_type: ItemType,
    pub enchantments: Vec<Enchantment>,
    pub anvil_use_count: i8,
}