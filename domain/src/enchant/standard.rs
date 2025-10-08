use std::rc::Rc;
use bon::builder;
use crate::combine::enchantments;
use crate::enchant::{Agnostic, Enchant, RequireCompatibleEnchantments, RequireCompatibleItemKind};
use crate::enchant::enchantments_combined::EnchantmentsCombined;
use crate::enchant::fallback::WithFallback;

#[builder]
pub fn standard(
    enchantments_combine_strategy: Rc<impl enchantments::Combine>,
    item_enchantment_compatibility: Rc<impl crate::item::EnchantmentCompatible>,
    enchantment_compatibility: Rc<impl crate::enchantment::Compatible>,
) -> impl Enchant {
    EnchantmentsCombined::with_strategy(enchantments_combine_strategy)
        .with_fallback(Agnostic)
        .require_compatible_item_kind(item_enchantment_compatibility)
        .require_compatible_enchantments(enchantment_compatibility)
}