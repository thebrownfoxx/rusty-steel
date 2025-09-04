use super::ItemEnchantmentCompatible;
use crate::enchantment::EnchantmentKindId;
use crate::item::ItemKindId;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ItemEnchantmentCompatibilityMatrix(pub HashMap<ItemKindId, Vec<EnchantmentKindId>>);

impl ItemEnchantmentCompatible for ItemEnchantmentCompatibilityMatrix {
    fn are_compatible(&self, item: ItemKindId, enchantment: EnchantmentKindId) -> bool {
        match self.0.get(&item) {
            None => false,
            Some(compatible) => compatible.contains(&enchantment),
        }
    }
}
