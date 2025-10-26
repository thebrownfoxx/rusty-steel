use super::{CombineEnchantments, CombineEnchantmentsError, CombineEnchantmentsResult};
use crate::enchantment::{Enchantment, MaxLevel};

pub struct RejectLevelOverflowCombineEnchantments<Impl: CombineEnchantments, Max: MaxLevel> {
    implementation: Impl,
    max_level_provider: Max,
}

impl<Impl: CombineEnchantments, Cap: MaxLevel> CombineEnchantments
    for RejectLevelOverflowCombineEnchantments<Impl, Cap>
{
    fn combine(
        &self,
        target: &mut Enchantment,
        sacrifice: &Enchantment,
    ) -> CombineEnchantmentsResult {
        let original_level = target.level;

        let () = self.implementation.combine(target, sacrifice)?;

        let Some(max_level) = self.max_level_provider.max_level(&target) else {
            return Ok(());
        };

        if target.level <= max_level {
            return Ok(());
        }

        target.level = original_level;
        Err(CombineEnchantmentsError::LevelsIncompatible)
    }
}

pub trait RejectLevelOverflow<Impl: CombineEnchantments, Max: MaxLevel> {
    fn reject_level_overflow(
        self,
        max_level_provider: Max,
    ) -> RejectLevelOverflowCombineEnchantments<Impl, Max>;
}

impl<Impl: CombineEnchantments, Max: MaxLevel> RejectLevelOverflow<Impl, Max> for Impl {
    fn reject_level_overflow(
        self,
        max_level_provider: Max,
    ) -> RejectLevelOverflowCombineEnchantments<Impl, Max> {
        RejectLevelOverflowCombineEnchantments {
            implementation: self,
            max_level_provider,
        }
    }
}
