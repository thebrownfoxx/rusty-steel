use crate::edition::{CloneByEdition, Edition};
use crate::enchantment::enchantment_kind::EnchantmentKind;
use crate::enchantment::enchantment_kinds::OwnedEnchantmentKinds;
use crate::enchantment::shared_enchantment_kind::SharedEnchantmentKind;

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
