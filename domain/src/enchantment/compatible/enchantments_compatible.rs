use crate::enchantment::EnchantmentKindId;

pub trait EnchantmentsCompatible {
    fn are_compatible(
        &self,
        enchantment_a: &dyn AsRef<EnchantmentKindId>,
        enchantment_b: &dyn AsRef<EnchantmentKindId>,
    ) -> bool;
}
