mod max_level_capped;
mod reject_level_overflow;
mod reject_lower_level_sacrifice;
mod standard;

pub use max_level_capped::{MaxLevelCapped, MaxLevelCappedBuilder};
pub use reject_level_overflow::{RejectLevelOverflow, RejectLevelOverflowBuilder};
pub use reject_lower_level_sacrifice::{
    RejectLowerLevelSacrifice, RejectLowerLevelSacrificeBuilder,
};
pub use standard::StandardEnchantmentCombiner;

use crate::enchantment::level::EnchantmentLevel;
use crate::enchantment::EnchantmentKindId;
use std::ops::Deref;

pub trait CombineEnchantmentLevels {
    fn combine(
        &self,
        kind: &EnchantmentKindId,
        target: EnchantmentLevel,
        sacrifice: EnchantmentLevel,
    ) -> Option<EnchantmentLevel>;
}

impl<Wrapper: Deref<Target: CombineEnchantmentLevels>> CombineEnchantmentLevels for Wrapper {
    fn combine(
        &self,
        kind: &EnchantmentKindId,
        target: EnchantmentLevel,
        sacrifice: EnchantmentLevel,
    ) -> Option<EnchantmentLevel> {
        self.deref().combine(kind, target, sacrifice)
    }
}
