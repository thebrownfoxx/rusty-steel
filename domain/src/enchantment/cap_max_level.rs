use crate::enchantment::{Enchantment, EnchantmentKinds};
use std::cmp::min;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;

pub trait CapMaxLevel {
    fn cap_max_level(&self, enchantment: &mut Enchantment);
}

impl<Kinds: EnchantmentKinds> CapMaxLevel for Kinds {
    fn cap_max_level(&self, enchantment: &mut Enchantment) {
        let Some(kind) = self.get(enchantment) else {
            return;
        };

        enchantment.level = min(enchantment.level, kind.max_level);
    }
}

impl<Impl: CapMaxLevel> CapMaxLevel for Rc<Impl> {
    fn cap_max_level(&self, enchantment: &mut Enchantment) {
        self.deref().cap_max_level(enchantment)
    }
}

impl<Impl: CapMaxLevel> CapMaxLevel for Arc<Impl> {
    fn cap_max_level(&self, enchantment: &mut Enchantment) {
        self.deref().cap_max_level(enchantment)
    }
}
