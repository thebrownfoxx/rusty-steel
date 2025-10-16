use super::{Combine, Error, MaxLevelCapped, RejectLowerLevelSacrifice, Result};
use crate::enchantment::{CapMaxLevel, Enchantment};
use std::cmp::max;

pub struct Standard;

impl Standard {
    pub fn bedrock(cap_strategy: impl CapMaxLevel) -> impl Combine {
        MaxLevelCapped {
            implementation: RejectLowerLevelSacrifice(Standard),
            cap_strategy,
        }
    }

    pub fn java() -> impl Combine {
        Standard
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
