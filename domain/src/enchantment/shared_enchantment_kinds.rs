use super::{EnchantmentKind, OwnedEnchantmentKinds, SharedEnchantmentKind};
use crate::edition::{CloneByEdition, Edition};

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub struct SharedEnchantmentKinds(pub Vec<SharedEnchantmentKind>);

impl CloneByEdition<OwnedEnchantmentKinds> for SharedEnchantmentKinds {
    fn clone_by_edition(&self, edition: Edition) -> OwnedEnchantmentKinds {
        OwnedEnchantmentKinds(self.contents_by_edition(edition))
    }
}

impl SharedEnchantmentKinds {
    fn contents_by_edition(&self, edition: Edition) -> Vec<EnchantmentKind> {
        self.0
            .iter()
            .map(|shared| shared.clone_by_edition(edition))
            .collect()
    }
}

impl From<Vec<SharedEnchantmentKind>> for SharedEnchantmentKinds {
    fn from(value: Vec<SharedEnchantmentKind>) -> Self {
        SharedEnchantmentKinds(value)
    }
}
