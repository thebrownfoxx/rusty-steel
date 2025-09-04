use crate::enchantment::compatible::enchantments_compatible::EnchantmentsCompatible;
use crate::enchantment::enchantment_kind::EnchantmentKindId;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct EnchantmentIncompatibilityMatrix(pub HashMap<EnchantmentKindId, Vec<EnchantmentKindId>>);

impl EnchantmentsCompatible for EnchantmentIncompatibilityMatrix {
    fn are_compatible(
        &self,
        enchantment_a: EnchantmentKindId,
        enchantment_b: EnchantmentKindId,
    ) -> bool {
        match self.0.get(&enchantment_a.into()) {
            None => true,
            Some(incompatible) => incompatible.contains(&enchantment_b.into()),
        }
    }
}
