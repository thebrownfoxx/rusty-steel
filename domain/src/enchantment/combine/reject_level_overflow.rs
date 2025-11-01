use super::{CombineEnchantments, CombineEnchantmentsError, CombineEnchantmentsResult};
use crate::builder::Builder;
use crate::enchantment::level::EnchantmentLevel;
use crate::enchantment::{Enchantment, EnchantmentKindId};

#[derive(Debug)]
pub struct RejectLevelOverflow<Impl, Max>
where
    Impl: CombineEnchantments,
    Max: Fn(&EnchantmentKindId) -> EnchantmentLevel,
{
    implementation: Impl,
    max_level: Max,
}

impl<Impl, Max> CombineEnchantments for RejectLevelOverflow<Impl, Max>
where
    Impl: CombineEnchantments,
    Max: Fn(&EnchantmentKindId) -> EnchantmentLevel,
{
    fn combine(
        &self,
        target: &mut Enchantment,
        sacrifice: &Enchantment,
    ) -> CombineEnchantmentsResult {
        let original_level = target.level;

        let () = self.implementation.combine(target, sacrifice)?;

        let max_level = (self.max_level)(&target.kind);

        if target.level <= max_level {
            return Ok(());
        }

        target.level = original_level;
        Err(CombineEnchantmentsError::LevelsIncompatible)
    }
}

pub trait RejectLevelOverflowBuilder<Impl, Max>
where
    Impl: CombineEnchantments,
    Max: Fn(&EnchantmentKindId) -> EnchantmentLevel,
{
    fn reject_level_overflow(self, max_level: Max) -> Builder<RejectLevelOverflow<Impl, Max>>;
}

impl<Impl, Max> RejectLevelOverflowBuilder<Impl, Max> for Builder<Impl>
where
    Impl: CombineEnchantments,
    Max: Fn(&EnchantmentKindId) -> EnchantmentLevel,
{
    fn reject_level_overflow(self, max_level: Max) -> Builder<RejectLevelOverflow<Impl, Max>> {
        self.reimplement(|implementation| RejectLevelOverflow {
            implementation,
            max_level,
        })
    }
}
