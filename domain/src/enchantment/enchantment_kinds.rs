use crate::enchantment::enchantment_kind::{EnchantmentKind, EnchantmentKindId};

pub trait EnchantmentKinds {
    fn all_ids(&self) -> impl Iterator<Item = EnchantmentKindId>;
    fn get(&self, id: EnchantmentKindId) -> Option<&EnchantmentKind>;
}

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub struct OwnedEnchantmentKinds(pub Vec<EnchantmentKind>);

impl EnchantmentKinds for OwnedEnchantmentKinds {
    fn all_ids(&self) -> impl Iterator<Item = EnchantmentKindId> {
        self.0.iter().map(|kind| kind.id)
    }

    fn get(&self, id: EnchantmentKindId) -> Option<&EnchantmentKind> {
        self.0.iter().find(|kind| kind.id == id)
    }
}
