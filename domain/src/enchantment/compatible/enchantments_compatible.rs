use crate::enchantment::EnchantmentKindId;

pub trait EnchantmentsCompatible {
    fn are_compatible(
        &self,
        enchantment_a: EnchantmentKindId,
        enchantment_b: EnchantmentKindId,
    ) -> bool;
}
