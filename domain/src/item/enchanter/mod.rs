use crate::enchantment::Enchantment;
use crate::enchantment::enchantment_compatibility::EnchantmentCompatibility;
use crate::item::Item;
use crate::item::item_enchantment_compatibility::ItemEnchantmentCompatibility;
use crate::item::item_kind::ItemKindId;

pub trait ItemEnchanter {
    fn add_enchantment(&self, item: Item, enchantment: Enchantment) -> Option<Item>;

    fn enchant(&self, item: ItemKindId, enchantments: Vec<Enchantment>) -> Option<Item> {
        let mut result = Item::new(item);
        for enchantment in enchantments {
            result = self.add_enchantment(result, enchantment)?
        }
        Some(result)
    }
}

pub struct CompatibilityCheckingItemEnchanter<'a> {
    pub enchantment_compatibility: &'a dyn EnchantmentCompatibility,
    pub item_enchantment_compatibility: &'a dyn ItemEnchantmentCompatibility,
}

impl ItemEnchanter for CompatibilityCheckingItemEnchanter<'_> {
    fn add_enchantment(&self, item: Item, enchantment: Enchantment) -> Option<Item> {
        // self.item_enchantment_compatibility.are_compatible(item.kind, enchantment.)
        todo!()
    }
}
