use crate::item::item_kind::{ItemKind, ItemKindId};

pub trait ItemKindProvider {
    fn all_ids(&self) -> impl Iterator<Item = &ItemKindId>;
    fn get(&self, id: &ItemKindId) -> Option<&impl ItemKind>;
}

pub struct OwnedItemKindProvider<T: ItemKind>(pub Vec<T>);

impl<T: ItemKind> ItemKindProvider for OwnedItemKindProvider<T> {
    fn all_ids(&self) -> impl Iterator<Item = &ItemKindId> {
        self.0.iter().map(|kind| kind.id())
    }

    fn get(&self, id: &ItemKindId) -> Option<&impl ItemKind> {
        self.0.iter().find(|kind| kind.id() == id)
    }
}
