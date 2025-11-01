use super::{ItemKind, ItemKindId};
use std::ops::Deref;
use std::rc::Rc;

pub trait ItemKinds {
    fn all_ids(&self) -> impl Iterator<Item = &ItemKindId>;
    fn get(&self, id: &impl AsRef<ItemKindId>) -> Option<&ItemKind>;
}

#[derive(Debug)]
pub struct OwnedItemKinds(Rc<[ItemKind]>);

impl OwnedItemKinds {
    pub fn new(item_kinds: impl Into<Rc<[ItemKind]>>) -> Self {
        Self(item_kinds.into())
    }
}

impl ItemKinds for OwnedItemKinds {
    fn all_ids(&self) -> impl Iterator<Item = &ItemKindId> {
        self.0.iter().map(|kind| &kind.id)
    }

    fn get(&self, id: &impl AsRef<ItemKindId>) -> Option<&ItemKind> {
        self.0.iter().find(|kind| kind.id == *id.as_ref())
    }
}

impl<T: Into<Rc<[ItemKind]>>> From<T> for OwnedItemKinds {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl<Wrapper: Deref<Target: ItemKinds>> ItemKinds for Wrapper {
    fn all_ids(&self) -> impl Iterator<Item = &ItemKindId> {
        self.deref().all_ids()
    }

    fn get(&self, id: &impl AsRef<ItemKindId>) -> Option<&ItemKind> {
        self.deref().get(id)
    }
}
