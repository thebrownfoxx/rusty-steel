use super::{Combine, Error, MaxLevelCapped, RejectLowerLevelSacrifice, Result};
use crate::combine::enchantments::reject_level_overflow::RejectLevelOverflow;
use crate::enchantment::{CapMaxLevel, Enchantment, MaxLevel};
use std::cmp::max;

pub struct Standard;

impl Standard {
    pub fn bedrock(max_level_provider: impl MaxLevel) -> impl Combine {
        Standard
            .reject_lower_level_sacrifice()
            .reject_level_overflow(max_level_provider)
    }

    pub fn java(cap_strategy: impl CapMaxLevel) -> impl Combine {
        Standard.max_level_capped(cap_strategy)
    }
}

impl Combine for Standard {
    fn combine(&self, target: &mut Enchantment, sacrifice: &Enchantment) -> Result {
        if target.kind != sacrifice.kind {
            return Err(Error::EnchantmentKindsIncompatible);
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
