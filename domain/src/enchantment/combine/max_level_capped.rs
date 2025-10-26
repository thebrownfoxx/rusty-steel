use super::{CombineEnchantments, CombineEnchantmentsResult};
use crate::enchantment::{CapMaxLevel, Enchantment};

#[derive(Debug)]
pub struct MaxLevelCappedEnchantmentCombiner<Impl, Cap>
where
    Impl: CombineEnchantments,
    Cap: CapMaxLevel,
{
    implementation: Impl,
    cap_strategy: Cap,
}

impl<Impl, Cap> CombineEnchantments for MaxLevelCappedEnchantmentCombiner<Impl, Cap>
where
    Impl: CombineEnchantments,
    Cap: CapMaxLevel,
{
    fn combine(
        &self,
        target: &mut Enchantment,
        sacrifice: &Enchantment,
    ) -> CombineEnchantmentsResult {
        let result = self.implementation.combine(target, sacrifice);

        if result.is_ok() {
            self.cap_strategy.cap_max_level(target);
        }

        result
    }
}

pub trait MaxLevelCappedCombineEnchantments<Impl, Cap>
where
    Impl: CombineEnchantments,
    Cap: CapMaxLevel,
{
    fn max_level_capped(self, cap_strategy: Cap) -> MaxLevelCappedEnchantmentCombiner<Impl, Cap>;
}

impl<Impl, Cap> MaxLevelCappedCombineEnchantments<Impl, Cap> for Impl
where
    Impl: CombineEnchantments,
    Cap: CapMaxLevel,
{
    fn max_level_capped(self, cap_strategy: Cap) -> MaxLevelCappedEnchantmentCombiner<Impl, Cap> {
        MaxLevelCappedEnchantmentCombiner {
            implementation: self,
            cap_strategy,
        }
    }
}
