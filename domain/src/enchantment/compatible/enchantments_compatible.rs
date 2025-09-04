use crate::enchantment::enchantment_kind::EnchantmentKindId;

pub trait EnchantmentsCompatible {
    fn are_compatible(
        &self,
        enchantment_a: EnchantmentKindId,
        enchantment_b: EnchantmentKindId,
    ) -> bool;
}
