use super::{ItemKind, ItemKindId};
use crate::edition::{CloneByEdition, Edition, EditionShared};
use bon::Builder;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, Builder)]
pub struct SharedItemKind {
    #[builder(into)]
    pub id: ItemKindId,
    #[builder(into)]
    pub name: EditionShared<Rc<str>>,
    #[builder(default)]
    pub is_book: bool,
}

impl CloneByEdition<ItemKind> for SharedItemKind {
    fn clone_by_edition(&self, edition: Edition) -> ItemKind {
        ItemKind {
            id: self.id.clone(),
            name: self.name.clone_by_edition(edition),
            is_book: self.is_book,
        }
    }
}
