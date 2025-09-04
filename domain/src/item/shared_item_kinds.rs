use super::{ItemKind, OwnedItemKinds, SharedItemKind};
use crate::edition::{CloneByEdition, Edition};

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct SharedItemKinds(pub Vec<SharedItemKind>);

impl CloneByEdition<OwnedItemKinds> for SharedItemKinds {
    fn clone_by_edition(&self, edition: Edition) -> OwnedItemKinds {
        OwnedItemKinds(self.contents_by_edition(edition))
    }
}

impl SharedItemKinds {
    fn contents_by_edition(&self, edition: Edition) -> Vec<ItemKind> {
        self.0
            .iter()
            .map(|shared| shared.clone_by_edition(edition))
            .collect()
    }
}
