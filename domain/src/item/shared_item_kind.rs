use super::{ItemKind, ItemKindId};
use crate::edition::{CloneByEdition, Edition, EditionShared};
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct SharedItemKind {
    pub id: ItemKindId,
    pub name: EditionShared<String>,
    pub is_book: bool,
}

impl SharedItemKind {
    pub fn book(
        id: impl Into<ItemKindId>,
        name: impl Into<EditionShared<String>>,
    ) -> SharedItemKind {
        SharedItemKind {
            id: id.into(),
            name: name.into(),
            is_book: true,
        }
    }

    pub fn item(
        id: impl Into<ItemKindId>,
        name: impl Into<EditionShared<String>>,
    ) -> SharedItemKind {
        SharedItemKind {
            id: id.into(),
            name: name.into(),
            is_book: false,
        }
    }
}

impl CloneByEdition<ItemKind> for SharedItemKind {
    fn clone_by_edition(&self, edition: Edition) -> ItemKind {
        ItemKind {
            id: self.id,
            name: self.name.clone_by_edition(edition),
            is_book: self.is_book,
        }
    }
}
