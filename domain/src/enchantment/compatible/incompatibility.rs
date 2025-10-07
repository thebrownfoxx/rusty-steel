use super::AreCompatible;
use crate::enchantment::EnchantmentKindId;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct IncompatibilityMap(pub HashMap<EnchantmentKindId, Vec<EnchantmentKindId>>);

impl AreCompatible for IncompatibilityMap {
    fn are_compatible(
        &self,
        enchantment_a: &impl AsRef<EnchantmentKindId>,
        enchantment_b: &impl AsRef<EnchantmentKindId>,
    ) -> bool {
        match self.0.get(enchantment_a.as_ref()) {
            None => true,
            Some(incompatible) => incompatible.contains(enchantment_b.as_ref()),
        }
    }
}
