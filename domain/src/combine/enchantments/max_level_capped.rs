use super::{Combine, Result};
use crate::enchantment::{CapMaxLevel, Enchantment};

pub struct MaxLevelCapped<Impl: Combine, Cap: CapMaxLevel> {
    pub implementation: Impl,
    pub cap_strategy: Cap,
}

impl<Impl: Combine, Cap: CapMaxLevel> Combine for MaxLevelCapped<Impl, Cap> {
    fn combine(&self, target: &mut Enchantment, sacrifice: &Enchantment) -> Result {
        let result = self.implementation.combine(target, sacrifice);

        if result.is_ok() {
            self.cap_strategy.cap_max_level(target);
        }

        result
    }
}
