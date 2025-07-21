use crate::enchantment::enchantment::Enchantment;
use crate::item::item_type::ItemType;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Item {
    item_type: ItemType,
    enchantments: Vec<Enchantment>,
    anvil_use_count: i8,
}

impl Item {
    pub fn new(
        item_type: ItemType,
        enchantments: Vec<Enchantment>,
        anvil_use_count: i8,
    ) -> Option<Item> {
        if enchantments.iter().any(|enchantment| {
            !item_type
                .compatible_enchantment_types
                .contains(&enchantment.enchantment_type.id)
        }) {
            None
        } else {
            Some(Item {
                item_type,
                enchantments,
                anvil_use_count,
            })
        }
    }
}
