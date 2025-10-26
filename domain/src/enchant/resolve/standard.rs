use crate::compatible::AreCompatible;
use crate::enchant::resolve::{
    Merge, RequireCompatible, ResolveEnchantments, UnhandledResolveEnchantments,
};
use crate::enchantment::combine::CombineEnchantments;
use crate::enchantment::EnchantmentKindId;

pub fn standard_resolver(
    combiner: impl CombineEnchantments,
    compatibility: impl AreCompatible<EnchantmentKindId>,
) -> impl ResolveEnchantments {
    UnhandledResolveEnchantments
        .merge(combiner)
        .require_compatible(compatibility)
}
