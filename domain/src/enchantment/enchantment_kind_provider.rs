use serde::{Deserialize, Serialize};
use crate::enchantment::enchantment_kind::{EnchantmentKind, EnchantmentKindId};

pub trait EnchantmentKindProvider {
    fn all_ids(&self) -> impl Iterator<Item = &EnchantmentKindId>;
    fn get(&self, id: &EnchantmentKindId) -> Option<&EnchantmentKind>;
}

#[derive(Eq, PartialEq, Clone, Hash, Debug, Serialize, Deserialize)]
pub struct OwnedEnchantmentKindProvider(pub Vec<EnchantmentKind>);

impl EnchantmentKindProvider for OwnedEnchantmentKindProvider {
    fn all_ids(&self) -> impl Iterator<Item = &EnchantmentKindId> {
        self.0.iter().map(|kind| &kind.id)
    }

    fn get(&self, id: &EnchantmentKindId) -> Option<&EnchantmentKind> {
        self.0.iter().find(|kind| &kind.id == id)
    }
}
