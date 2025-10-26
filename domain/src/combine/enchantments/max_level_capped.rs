use super::{Combine, Result};
use crate::enchantment::{CapMaxLevel, Enchantment};

pub struct MaxLevelCappedCombine<Impl: Combine, Cap: CapMaxLevel> {
    implementation: Impl,
    cap_strategy: Cap,
}

impl<Impl: Combine, Cap: CapMaxLevel> Combine for MaxLevelCappedCombine<Impl, Cap> {
    fn combine(&self, target: &mut Enchantment, sacrifice: &Enchantment) -> Result {
        let result = self.implementation.combine(target, sacrifice);

        if result.is_ok() {
            self.cap_strategy.cap_max_level(target);
        }

        result
    }
}

pub trait MaxLevelCapped<Impl: Combine, Cap: CapMaxLevel> {
    fn max_level_capped(self, cap_strategy: Cap) -> MaxLevelCappedCombine<Impl, Cap>;
}

impl<Impl: Combine, Cap: CapMaxLevel> MaxLevelCapped<Impl, Cap> for Impl {
    fn max_level_capped(self, cap_strategy: Cap) -> MaxLevelCappedCombine<Impl, Cap> {
        MaxLevelCappedCombine {
            implementation: self,
            cap_strategy,
        }
    }
}
