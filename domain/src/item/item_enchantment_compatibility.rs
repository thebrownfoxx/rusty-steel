use crate::enchantment::enchantment_kind::EnchantmentKindId;
use crate::item::item_kind::ItemKindId;
use std::collections::HashMap;

pub trait ItemEnchantmentCompatibility {
    fn are_compatible(&self, item: &ItemKindId, enchantment: &EnchantmentKindId) -> bool;
}

pub struct ItemEnchantmentCompatibilityMatrix(pub HashMap<ItemKindId, Vec<EnchantmentKindId>>);

impl ItemEnchantmentCompatibility for ItemEnchantmentCompatibilityMatrix {
    fn are_compatible(&self, item: &ItemKindId, enchantment: &EnchantmentKindId) -> bool {
        match self.0.get(item) {
            None => false,
            Some(compatible) => compatible.contains(enchantment),
        }
    }
}
