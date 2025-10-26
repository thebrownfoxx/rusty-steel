use super::{CombineEnchantments, CombineEnchantmentsError, CombineEnchantmentsResult};
use crate::enchantment::{Enchantment, MaxLevel};

#[derive(Debug)]
pub struct RejectLevelOverflowEnchantmentCombiner<Impl, Max>
where
    Impl: CombineEnchantments,
    Max: MaxLevel,
{
    implementation: Impl,
    max_level_provider: Max,
}

impl<Impl, Cap> CombineEnchantments for RejectLevelOverflowEnchantmentCombiner<Impl, Cap>
where
    Impl: CombineEnchantments,
    Cap: MaxLevel,
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

pub trait RejectLevelOverflowCombineEnchantments<Impl, Max>
where
    Impl: CombineEnchantments,
    Max: MaxLevel,
{
    fn reject_level_overflow(
        self,
        max_level_provider: Max,
    ) -> RejectLevelOverflowEnchantmentCombiner<Impl, Max>;
}

impl<Impl, Max> RejectLevelOverflowCombineEnchantments<Impl, Max> for Impl
where
    Impl: CombineEnchantments,
    Max: MaxLevel,
{
    fn reject_level_overflow(
        self,
        max_level_provider: Max,
    ) -> RejectLevelOverflowEnchantmentCombiner<Impl, Max> {
        RejectLevelOverflowEnchantmentCombiner {
            implementation: self,
            max_level_provider,
        }
    }
}
