use crate::edition::edition_shared::EditionShared;
use crate::edition::{CloneByEdition, Edition};
use crate::enchantment::enchantment_kind::EnchantmentKindId;
use crate::item::item_kind::{ItemKind, ItemKindId};
use std::collections::HashSet;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct SharedItemKind {
    pub id: ItemKindId,
    pub name: EditionShared<String>,
    pub compatible_enchantments: EditionShared<HashSet<EnchantmentKindId>>,
}

impl SharedItemKind {
    pub fn new(
        id: impl Into<ItemKindId>,
        name: impl Into<EditionShared<String>>,
        compatible_enchantments: impl Into<EditionShared<HashSet<EnchantmentKindId>>>,
    ) -> SharedItemKind {
        SharedItemKind {
            id: id.into(),
            name: name.into(),
            compatible_enchantments: compatible_enchantments.into(),
        }
    }
}

impl CloneByEdition<ItemKind> for SharedItemKind {
    fn clone_by_edition(&self, edition: Edition) -> ItemKind {
        ItemKind::new(
            self.id.clone(),
            self.name.clone_by_edition(edition),
            self.compatible_enchantments.clone_by_edition(edition),
        )
    }
}
