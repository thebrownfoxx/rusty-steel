use crate::enchantment::{EnchantmentKindId, EnchantmentKinds};
use std::ops::Deref;

pub trait MaxLevel {
    fn max_level(&self, enchantment: impl AsRef<EnchantmentKindId>) -> Option<u8>;
}

impl<Wrapper> MaxLevel for Wrapper
where
    Wrapper: Deref<Target: MaxLevel>,
{
    fn max_level(&self, enchantment: impl AsRef<EnchantmentKindId>) -> Option<u8> {
        self.deref().max_level(enchantment)
    }
}

#[derive(Debug)]
pub struct EnchantmentKindsMaxLevel<Kinds: EnchantmentKinds> {
    enchantment_kinds: Kinds,
}

impl<Kinds: EnchantmentKinds> MaxLevel for EnchantmentKindsMaxLevel<Kinds> {
    fn max_level(&self, enchantment: impl AsRef<EnchantmentKindId>) -> Option<u8> {
        Some(self.enchantment_kinds.get(enchantment.as_ref())?.max_level)
    }
}

impl<Kinds: EnchantmentKinds> From<Kinds> for EnchantmentKindsMaxLevel<Kinds> {
    fn from(value: Kinds) -> Self {
        Self {
            enchantment_kinds: value,
        }
    }
}
