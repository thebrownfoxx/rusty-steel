use super::EnchantmentsCompatible;
use crate::enchantment::EnchantmentKindId;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct EnchantmentIncompatibilityMatrix(pub HashMap<EnchantmentKindId, Vec<EnchantmentKindId>>);

impl EnchantmentsCompatible for EnchantmentIncompatibilityMatrix {
    fn are_compatible(
        &self,
        enchantment_a: &dyn AsRef<EnchantmentKindId>,
        enchantment_b: &dyn AsRef<EnchantmentKindId>,
    ) -> bool {
        match self.0.get(enchantment_a.as_ref()) {
            None => true,
            Some(incompatible) => incompatible.contains(enchantment_b.as_ref()),
        }
    }
}
