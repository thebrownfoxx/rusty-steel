use crate::enchantment::{Enchantment, MaxLevel};
use std::cmp::min;
use std::ops::Deref;

pub trait CapMaxLevel {
    fn cap_max_level(&self, enchantment: &mut Enchantment);
}

impl<Wrapper> CapMaxLevel for Wrapper
where
    Wrapper: Deref<Target: CapMaxLevel>,
{
    fn cap_max_level(&self, enchantment: &mut Enchantment) {
        self.deref().cap_max_level(enchantment)
    }
}

pub struct MaxLevelCapMaxLevel<Max: MaxLevel> {
    max_level_provider: Max,
}

impl<Max: MaxLevel> CapMaxLevel for MaxLevelCapMaxLevel<Max> {
    fn cap_max_level(&self, enchantment: &mut Enchantment) {
        let Some(max_level) = self.max_level_provider.max_level(&enchantment) else {
            return;
        };

        enchantment.level = min(max_level, enchantment.level);
    }
}

impl<Max: MaxLevel> From<Max> for MaxLevelCapMaxLevel<Max> {
    fn from(value: Max) -> Self {
        Self {
            max_level_provider: value,
        }
    }
}
