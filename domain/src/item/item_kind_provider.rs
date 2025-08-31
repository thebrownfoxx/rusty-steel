use crate::item::item_kind::{ItemKind, ItemKindId};

pub trait ItemKindProvider {
    fn all_ids(&self) -> impl Iterator<Item = ItemKindId>;
    fn get(&self, id: ItemKindId) -> Option<&ItemKind>;
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct OwnedItemKindProvider(pub Vec<ItemKind>);

impl ItemKindProvider for OwnedItemKindProvider {
    fn all_ids(&self) -> impl Iterator<Item = ItemKindId> {
        self.0.iter().map(|kind| kind.id)
    }

    fn get(&self, id: ItemKindId) -> Option<&ItemKind> {
        self.0.iter().find(|kind| kind.id == id)
    }
}
