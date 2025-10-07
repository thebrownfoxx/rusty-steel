use super::{EnchantmentKind, EnchantmentKindId};

pub trait EnchantmentKinds {
    fn all_ids(&self) -> impl Iterator<Item = &EnchantmentKindId>;
    fn get(&self, id: &impl AsRef<EnchantmentKindId>) -> Option<&EnchantmentKind>;
}

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub struct OwnedEnchantmentKinds(pub Vec<EnchantmentKind>);

impl EnchantmentKinds for OwnedEnchantmentKinds {
    fn all_ids(&self) -> impl Iterator<Item = &EnchantmentKindId> {
        self.0.iter().map(|kind| &kind.id)
    }

    fn get(&self, id: &impl AsRef<EnchantmentKindId>) -> Option<&EnchantmentKind> {
        self.0.iter().find(|kind| kind.id == *id.as_ref())
    }
}

impl From<Vec<EnchantmentKind>> for OwnedEnchantmentKinds {
    fn from(value: Vec<EnchantmentKind>) -> Self {
        OwnedEnchantmentKinds(value)
    }
}
