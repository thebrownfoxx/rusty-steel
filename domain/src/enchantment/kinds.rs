use super::{EnchantmentKind, EnchantmentKindId};
use std::fmt::Debug;
use std::ops::Deref;
use std::rc::Rc;

pub trait EnchantmentKinds {
    fn all_ids(&self) -> impl Iterator<Item = &EnchantmentKindId>;
    fn get(&self, id: &impl AsRef<EnchantmentKindId>) -> Option<&EnchantmentKind>;
}

#[derive(Debug)]
pub struct OwnedEnchantmentKinds(Rc<[EnchantmentKind]>);

impl OwnedEnchantmentKinds {
    pub fn new(enchantment_kinds: impl Into<Rc<[EnchantmentKind]>>) -> Self {
        Self(enchantment_kinds.into())
    }
}

impl EnchantmentKinds for OwnedEnchantmentKinds {
    fn all_ids(&self) -> impl Iterator<Item = &EnchantmentKindId> {
        self.0.iter().map(|kind| &kind.id)
    }

    fn get(&self, id: &impl AsRef<EnchantmentKindId>) -> Option<&EnchantmentKind> {
        self.0.iter().find(|kind| kind.id == *id.as_ref())
    }
}

impl<T: Into<Rc<[EnchantmentKind]>>> From<T> for OwnedEnchantmentKinds {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl<Wrapper: Deref<Target: EnchantmentKinds>> EnchantmentKinds for Wrapper {
    fn all_ids(&self) -> impl Iterator<Item = &EnchantmentKindId> {
        self.deref().all_ids()
    }

    fn get(&self, id: &impl AsRef<EnchantmentKindId>) -> Option<&EnchantmentKind> {
        self.deref().get(id)
    }
}
