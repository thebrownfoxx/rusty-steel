use crate::item::item_kind::{ItemKind, ItemKindId};

pub trait ItemKindProvider {
    fn all_ids(&self) -> impl Iterator<Item = &ItemKindId>;
    fn get(&self, id: &ItemKindId) -> &impl ItemKind;
}
