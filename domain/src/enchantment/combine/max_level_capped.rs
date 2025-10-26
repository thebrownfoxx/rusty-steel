use super::{CombineEnchantments, CombineEnchantmentsResult};
use crate::enchantment::{CapMaxLevel, Enchantment};

pub struct MaxLevelCappedCombineEnchantments<Impl: CombineEnchantments, Cap: CapMaxLevel> {
    implementation: Impl,
    cap_strategy: Cap,
}

impl<Impl: CombineEnchantments, Cap: CapMaxLevel> CombineEnchantments
    for MaxLevelCappedCombineEnchantments<Impl, Cap>
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

pub trait MaxLevelCapped<Impl: CombineEnchantments, Cap: CapMaxLevel> {
    fn max_level_capped(self, cap_strategy: Cap) -> MaxLevelCappedCombineEnchantments<Impl, Cap>;
}

impl<Impl: CombineEnchantments, Cap: CapMaxLevel> MaxLevelCapped<Impl, Cap> for Impl {
    fn max_level_capped(self, cap_strategy: Cap) -> MaxLevelCappedCombineEnchantments<Impl, Cap> {
        MaxLevelCappedCombineEnchantments {
            implementation: self,
            cap_strategy,
        }
    }
}
