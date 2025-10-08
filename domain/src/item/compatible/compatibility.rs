use super::EnchantmentCompatible;
use crate::enchantment::EnchantmentKindId;
use crate::item::ItemKindId;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct EnchantmentCompatibilityMap(pub HashMap<ItemKindId, Vec<EnchantmentKindId>>);

impl EnchantmentCompatible for EnchantmentCompatibilityMap {
    fn are_compatible(
        &self,
        item: &dyn AsRef<ItemKindId>,
        enchantment: &dyn AsRef<EnchantmentKindId>,
    ) -> bool {
        match self.0.get(item.as_ref()) {
            None => false,
            Some(compatible) => compatible.contains(enchantment.as_ref()),
        }
    }
}
