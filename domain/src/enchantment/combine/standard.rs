use super::{
    CombineEnchantments, CombineEnchantmentsError, CombineEnchantmentsResult,
    MaxLevelCappedCombineEnchantments, RejectLowerLevelSacrificeCombineEnchantments,
};
use crate::enchantment::combine::reject_level_overflow::RejectLevelOverflowCombineEnchantments;
use crate::enchantment::{CapMaxLevel, Enchantment, MaxLevel};
use std::cmp::max;

#[derive(Debug)]
pub struct StandardEnchantmentCombiner;

impl StandardEnchantmentCombiner {
    pub fn bedrock(max_level_provider: impl MaxLevel) -> impl CombineEnchantments {
        StandardEnchantmentCombiner
            .reject_lower_level_sacrifice()
            .reject_level_overflow(max_level_provider)
    }

    pub fn java(cap_strategy: impl CapMaxLevel) -> impl CombineEnchantments {
        StandardEnchantmentCombiner.max_level_capped(cap_strategy)
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

        target.level = combined_level(target.level, sacrifice.level);
        Ok(())
    }
}

fn combined_level(target_level: u8, sacrifice_level: u8) -> u8 {
    if target_level == sacrifice_level {
        return target_level + 1;
    }

    max(target_level, sacrifice_level)
}
