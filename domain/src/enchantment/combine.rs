use crate::enchantment::level::EnchantmentLevel;
use crate::enchantment::EnchantmentKindId;
use std::cmp::min;

pub struct CombineEnchantments;

impl CombineEnchantments {
    pub fn bedrock(
        kind: &EnchantmentKindId,
        target: EnchantmentLevel,
        sacrifice: EnchantmentLevel,
        max_level: impl Fn(&EnchantmentKindId) -> Option<EnchantmentLevel>,
    ) -> Option<EnchantmentLevel> {
        if sacrifice < target {
            return None;
        }

        let level = target.combine(sacrifice);

        if max_level(kind)? < level {
            return None;
        }

        Some(level)
    }

    pub fn java(
        kind: &EnchantmentKindId,
        target: EnchantmentLevel,
        sacrifice: EnchantmentLevel,
        max_level: impl Fn(&EnchantmentKindId) -> Option<EnchantmentLevel>,
    ) -> Option<EnchantmentLevel> {
        let level = target.combine(sacrifice);
        let max_level = max_level(kind)?;
        Some(min(level, max_level))
    }
}
