use super::IncompatibilityMap;
use crate::edition::{CloneByEdition, Edition, EditionShared};
use crate::enchantment::EnchantmentKindId;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct SharedIncompatibilityMap(
    pub HashMap<EnchantmentKindId, EditionShared<Vec<EnchantmentKindId>>>,
);

impl CloneByEdition<IncompatibilityMap> for SharedIncompatibilityMap {
    fn clone_by_edition(&self, edition: Edition) -> IncompatibilityMap {
        IncompatibilityMap(self.contents_by_edition(edition))
    }
}

impl SharedIncompatibilityMap {
    fn contents_by_edition(
        &self,
        edition: Edition,
    ) -> HashMap<EnchantmentKindId, Vec<EnchantmentKindId>> {
        self.0.iter().map(clone_by_edition(edition)).collect()
    }
}

type SharedIncompatibleEnchantments<'a> = (
    &'a EnchantmentKindId,
    &'a EditionShared<Vec<EnchantmentKindId>>,
);

type IncompatibleEnchantments = (EnchantmentKindId, Vec<EnchantmentKindId>);

fn clone_by_edition(
    edition: Edition,
) -> impl FnMut(SharedIncompatibleEnchantments) -> IncompatibleEnchantments {
    move |(enchantment, incompatible)| (enchantment.clone(), incompatible.clone_by_edition(edition))
}
