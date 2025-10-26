use crate::compatible::AreCompatible;
use crate::enchant::agnostic::AgnosticEnchanter;
use crate::enchant::resolve::{standard_enchantment_resolver, ResolveEnchantmentsEnchant};
use crate::enchant::{Enchant, RequireCompatibleItemEnchant};
use crate::enchantment::combine::CombineEnchantments;
use crate::enchantment::EnchantmentKindId;
use crate::item::ItemKindId;

pub fn standard_enchanter(
    combiner: impl CombineEnchantments,
    enchantment_compatibility: impl AreCompatible<EnchantmentKindId>,
    item_compatibility: impl AreCompatible<ItemKindId, EnchantmentKindId>,
) -> impl Enchant {
    let resolver = standard_enchantment_resolver(combiner, enchantment_compatibility);
    AgnosticEnchanter
        .resolve_enchantments(resolver)
        .require_compatible_item(item_compatibility)
}
