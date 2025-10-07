use std::cmp::min;
use crate::enchantment::{Enchantment, EnchantmentKinds};

pub trait CapMaxLevel {
    fn cap_max_level(&self, enchantment: &mut Enchantment);
}

impl<Kinds> CapMaxLevel for Kinds
where Kinds: EnchantmentKinds {
    fn cap_max_level(&self, enchantment: &mut Enchantment) {
        if let Some(kind) = self.get(enchantment) {
            enchantment.level = min(enchantment.level, kind.max_level);
        }
    }
}
