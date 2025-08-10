use crate::edition::{CloneByEdition, Edition};
use crate::enchantment::enchantment_kind::EnchantmentKind;
use crate::enchantment::enchantment_kind_provider::OwnedEnchantmentKindProvider;
use crate::enchantment::shared_enchantment_kind::SharedEnchantmentKind;

pub struct SharedEnchantmentKindProvider(pub Vec<SharedEnchantmentKind>);

impl CloneByEdition<OwnedEnchantmentKindProvider> for SharedEnchantmentKindProvider {
    fn clone_by_edition(&self, edition: Edition) -> OwnedEnchantmentKindProvider {
        OwnedEnchantmentKindProvider(self.contents_by_edition(edition))
    }
}

impl SharedEnchantmentKindProvider {
    fn contents_by_edition(&self, edition: Edition) -> Vec<EnchantmentKind> {
        self.0
            .iter()
            .map(|shared| shared.clone_by_edition(edition))
            .collect()
    }
}
