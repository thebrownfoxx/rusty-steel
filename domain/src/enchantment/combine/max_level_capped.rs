use super::{CombineEnchantmentLevels, CombineEnchantmentsResult};
use crate::builder::Builder;
use crate::enchantment::level::EnchantmentLevel;
use crate::enchantment::{Enchantment, EnchantmentKindId};
use std::cmp::min;

#[derive(Debug)]
pub struct MaxLevelCapped<Impl, Max>
where
    Impl: CombineEnchantmentLevels,
    Max: Fn(&EnchantmentKindId) -> EnchantmentLevel,
{
    implementation: Impl,
    max_level: Max,
}

impl<Impl, Max> CombineEnchantmentLevels for MaxLevelCapped<Impl, Max>
where
    Impl: CombineEnchantmentLevels,
    Max: Fn(&EnchantmentKindId) -> EnchantmentLevel,
{
    fn combine(
        &self,
        target: &mut Enchantment,
        sacrifice: &Enchantment,
    ) -> CombineEnchantmentsResult {
        let result = self.implementation.combine(target, sacrifice);

        if result.is_ok() {
            let max_level = (self.max_level)(&target.kind);
            target.level = min(target.level, max_level)
        }

        result
    }
}

pub trait MaxLevelCappedBuilder<Impl, Max>
where
    Impl: CombineEnchantmentLevels,
    Max: Fn(&EnchantmentKindId) -> EnchantmentLevel,
{
    fn max_level_capped(self, max_level: Max) -> Builder<MaxLevelCapped<Impl, Max>>;
}

impl<Impl, Max> MaxLevelCappedBuilder<Impl, Max> for Builder<Impl>
where
    Impl: CombineEnchantmentLevels,
    Max: Fn(&EnchantmentKindId) -> EnchantmentLevel,
{
    fn max_level_capped(self, max_level: Max) -> Builder<MaxLevelCapped<Impl, Max>> {
        self.reimplement(|implementation| MaxLevelCapped {
            implementation,
            max_level,
        })
    }
}
