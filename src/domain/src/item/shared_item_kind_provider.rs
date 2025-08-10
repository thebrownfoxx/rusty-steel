use crate::edition::{CloneByEdition, Edition};
use crate::item::item_kind::ItemKind;
use crate::item::item_kind_provider::OwnedItemKindProvider;
use crate::item::shared_item_kind::SharedItemKind;

pub struct SharedItemKindProvider(pub Vec<SharedItemKind>);

impl CloneByEdition<OwnedItemKindProvider> for SharedItemKindProvider {
    fn clone_by_edition(&self, edition: Edition) -> OwnedItemKindProvider {
        OwnedItemKindProvider(self.contents_by_edition(edition))
    }
}

impl SharedItemKindProvider {
    fn contents_by_edition(&self, edition: Edition) -> Vec<ItemKind> {
        self.0
            .iter()
            .map(|shared| shared.clone_by_edition(edition))
            .collect()
    }
}