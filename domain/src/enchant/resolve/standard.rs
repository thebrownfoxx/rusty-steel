use crate::builder::Builder;
use crate::compatible::AreCompatible;
use crate::enchant::resolve::{
    MergeBuilder, RejectIncompatibleBuilder, ResolveEnchantments, UnhandledEnchantmentsResolver,
};
use crate::enchantment::combine::CombineEnchantments;
use crate::enchantment::EnchantmentKindId;

pub fn standard_enchantment_resolver(
    combiner: impl CombineEnchantments,
    compatibility: impl AreCompatible<EnchantmentKindId>,
) -> impl ResolveEnchantments {
    Builder::new(UnhandledEnchantmentsResolver)
        .merge(combiner)
        .reject_incompatible(compatibility)
        .build()
}
