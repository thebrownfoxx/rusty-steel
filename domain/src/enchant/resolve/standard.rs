use crate::compatible::AreCompatible;
use crate::enchant::resolve::{
    MergeResolveEnchantments, RejectIncompatibleResolveEnchantments, ResolveEnchantments,
    UnhandledEnchantmentResolver,
};
use crate::enchantment::EnchantmentKindId;
use crate::enchantment::combine::CombineEnchantments;

pub fn standard_enchantment_resolver(
    combiner: impl CombineEnchantments,
    compatibility: impl AreCompatible<EnchantmentKindId>,
) -> impl ResolveEnchantments {
    UnhandledEnchantmentResolver
        .merge(combiner)
        .reject_incompatible(compatibility)
}
