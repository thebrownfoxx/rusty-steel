use crate::edition::edition_shared::EditionShared;
use crate::edition::{CloneByEdition, Edition};
use crate::item::item_kind::{ItemKind, ItemKindId};

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct SharedItemKind {
    pub id: ItemKindId,
    pub name: EditionShared<String>,
}

impl SharedItemKind {
    pub fn new(
        id: impl Into<ItemKindId>,
        name: impl Into<EditionShared<String>>,
    ) -> SharedItemKind {
        SharedItemKind {
            id: id.into(),
            name: name.into(),
        }
    }
}

impl CloneByEdition<ItemKind> for SharedItemKind {
    fn clone_by_edition(&self, edition: Edition) -> ItemKind {
        ItemKind::new(
            self.id.clone(),
            self.name.clone_by_edition(edition),
        )
    }
}
