use super::{EnchantmentKind, EnchantmentKindId};
use std::fmt::Debug;
use std::ops::Deref;

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

impl<Wrapper> EnchantmentKinds for Wrapper
where
    Wrapper: Deref<Target: EnchantmentKinds>,
{
    fn all_ids(&self) -> impl Iterator<Item = &EnchantmentKindId> {
        self.deref().all_ids()
    }

    fn get(&self, id: &impl AsRef<EnchantmentKindId>) -> Option<&EnchantmentKind> {
        self.deref().get(id)
    }
}
