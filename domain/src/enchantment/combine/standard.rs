use super::{
    CombineEnchantments, CombineEnchantmentsError, CombineEnchantmentsResult,
    MaxLevelCappedBuilder, RejectLowerLevelSacrificeBuilder,
};
use crate::builder::Builder;
use crate::enchantment::combine::reject_level_overflow::RejectLevelOverflowBuilder;
use crate::enchantment::level::EnchantmentLevel;
use crate::enchantment::{Enchantment, EnchantmentKindId};

#[derive(Debug)]
pub struct StandardEnchantmentCombiner;

impl StandardEnchantmentCombiner {
    pub fn bedrock(
        max_level: impl Fn(&EnchantmentKindId) -> EnchantmentLevel,
    ) -> impl CombineEnchantments {
        Builder::new(StandardEnchantmentCombiner)
            .reject_lower_level_sacrifice()
            .reject_level_overflow(max_level)
            .build()
    }

    pub fn java(
        max_level: impl Fn(&EnchantmentKindId) -> EnchantmentLevel,
    ) -> impl CombineEnchantments {
        Builder::new(StandardEnchantmentCombiner)
            .max_level_capped(max_level)
            .build()
    }
}

impl CombineEnchantments for StandardEnchantmentCombiner {
    fn combine(
        &self,
        target: &mut Enchantment,
        sacrifice: &Enchantment,
    ) -> CombineEnchantmentsResult {
        if target.kind != sacrifice.kind {
            return Err(CombineEnchantmentsError::EnchantmentKindsIncompatible);
        }

        target.level = target.level.combine(sacrifice.level);
        Ok(())
    }
}
