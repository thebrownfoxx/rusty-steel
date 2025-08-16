use crate::enchantment::enchantment_kind::EnchantmentKindId;
use std::collections::HashMap;

pub trait EnchantmentCompatibility {
    fn are_compatible(
        &self,
        enchantment_a: &EnchantmentKindId,
        enchantment_b: &EnchantmentKindId,
    ) -> bool;
}

pub struct EnchantmentIncompatibilityMatrix(pub HashMap<EnchantmentKindId, Vec<EnchantmentKindId>>);

impl EnchantmentCompatibility for EnchantmentIncompatibilityMatrix {
    fn are_compatible(
        &self,
        enchantment_a: &EnchantmentKindId,
        enchantment_b: &EnchantmentKindId,
    ) -> bool {
        match self.0.get(enchantment_a) {
            None => true,
            Some(incompatible) => incompatible.contains(enchantment_b),
        }
    }
}
