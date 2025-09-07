use super::{ItemKind, ItemKindId};

pub trait ItemKinds {
    fn all_ids(&self) -> impl Iterator<Item = &ItemKindId>;
    fn get(&self, id: ItemKindId) -> Option<&ItemKind>;
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct OwnedItemKinds(pub Vec<ItemKind>);

impl ItemKinds for OwnedItemKinds {
    fn all_ids(&self) -> impl Iterator<Item = &ItemKindId> {
        self.0.iter().map(|kind| &kind.id)
    }

    fn get(&self, id: ItemKindId) -> Option<&ItemKind> {
        self.0.iter().find(|kind| kind.id == id)
    }
}
