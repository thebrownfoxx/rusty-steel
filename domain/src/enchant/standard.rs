use crate::compatible::AreCompatible;
use crate::enchant::agnostic::AgnosticEnchant;
use crate::enchant::resolve::{standard_resolver, ResolveAgainstTargetEnchantments};
use crate::enchant::{Enchant, RequireCompatibleItemKind};
use crate::enchantment::combine::CombineEnchantments;
use crate::enchantment::EnchantmentKindId;
use crate::item::ItemKindId;

pub fn standard(
    combiner: impl CombineEnchantments,
    enchantment_compatibility: impl AreCompatible<EnchantmentKindId>,
    item_compatibility: impl AreCompatible<ItemKindId, EnchantmentKindId>,
) -> impl Enchant {
    AgnosticEnchant
        .resolve_against_target_enchantments(standard_resolver(combiner, enchantment_compatibility))
        .require_compatible_item_kind(item_compatibility)
}
