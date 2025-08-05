use crate::enchantment::enchantment_kind::{EnchantmentKind, EnchantmentKindId};

pub trait EnchantmentKindProvider {
    fn all_ids(&self) -> impl Iterator<Item = &EnchantmentKindId>;
    fn get(&self, id: &EnchantmentKindId) -> Option<&impl EnchantmentKind>;
}

pub struct OwnedEnchantmentKindProvider<T: EnchantmentKind>(pub Vec<T>);

impl<T: EnchantmentKind> EnchantmentKindProvider for OwnedEnchantmentKindProvider<T> {
    fn all_ids(&self) -> impl Iterator<Item = &EnchantmentKindId> {
        self.0.iter().map(|kind| kind.id())
    }

    fn get(&self, id: &EnchantmentKindId) -> Option<&impl EnchantmentKind> {
        self.0.iter().find(|kind| kind.id() == id)
    }
}
