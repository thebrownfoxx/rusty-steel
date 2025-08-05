use crate::enchantment::enchantment_kind::EnchantmentKind;

pub mod cost_multiplier;
pub mod enchantment_kind;
pub mod enchantment_kind_provider;

pub trait Enchantment {
    fn kind(&self) -> &impl EnchantmentKind;
    fn level(&self) -> u8;
}

pub struct OwnedEnchantment<T: EnchantmentKind> {
    pub kind: T,
    level: u8,
}

impl<T: EnchantmentKind> OwnedEnchantment<T> {
    fn new(kind: T, level: u8) -> Option<Self> {
        if level > kind.max_level() {
            return None;
        }
        Some(OwnedEnchantment { kind, level })
    }
}

impl<T: EnchantmentKind> Enchantment for OwnedEnchantment<T> {
    fn kind(&self) -> &impl EnchantmentKind {
        &self.kind
    }

    fn level(&self) -> u8 {
        self.level
    }
}
