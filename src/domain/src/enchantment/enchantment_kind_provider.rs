use crate::enchantment::enchantment_kind::{EnchantmentKind, EnchantmentKindId};

pub trait EnchantmentKindProvider {
    fn all_ids(&self) -> impl Iterator<Item = &EnchantmentKindId>;
    fn get(&self, id: &EnchantmentKindId) -> Option<&impl EnchantmentKind>;
}

pub struct OwnedEnchantmentKindProvider<T: EnchantmentKind> {
    pub enchantment_kinds: Vec<T>,
}

impl<T: EnchantmentKind> EnchantmentKindProvider for OwnedEnchantmentKindProvider<T> {
    fn all_ids(&self) -> impl Iterator<Item=&EnchantmentKindId> {
        self.enchantment_kinds
            .iter()
            .map(|kind| kind.id())
    }

    fn get(&self, id: &EnchantmentKindId) -> Option<&impl EnchantmentKind> {
        self.enchantment_kinds
            .iter()
            .find(|kind| kind.id() == id)
    }
}