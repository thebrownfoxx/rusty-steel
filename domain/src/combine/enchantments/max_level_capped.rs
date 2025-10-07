use super::{Combine, Result};
use crate::enchantment::{CapMaxLevel, Enchantment, EnchantmentKinds};
use std::rc::Rc;

pub struct MaxLevelCapped<Impl, Cap>
where
    Impl: Combine,
    Cap: CapMaxLevel,
{
    implementation: Impl,
    cap_strategy: Rc<Cap>,
}

impl<Impl, Cap> Combine for MaxLevelCapped<Impl, Cap>
where
    Impl: Combine,
    Cap: CapMaxLevel,
{
    fn combine(&self, target: &mut Enchantment, sacrifice: Enchantment) -> Result {
        let result = self.implementation.combine(target, sacrifice);

        if result.is_ok() {
            self.cap_strategy.cap_max_level(target);
        }

        result
    }
}

pub trait WithMaxLevelCapped<Impl, Cap>
where
    Impl: Combine,
    Cap: CapMaxLevel,
{
    fn with_max_level_capped(self, cap_strategy: Rc<Cap>) -> MaxLevelCapped<Impl, Cap>;
}

impl<Impl, Cap> WithMaxLevelCapped<Impl, Cap> for Impl
where
    Impl: Combine,
    Cap: CapMaxLevel,
{
    fn with_max_level_capped(self, cap_strategy: Rc<Cap>) -> MaxLevelCapped<Impl, Cap> {
        MaxLevelCapped {
            implementation: self,
            cap_strategy,
        }
    }
}
