use super::EnchantmentCompatibilityMap;
use crate::edition::{CloneByEdition, Edition, EditionShared};
use crate::enchantment::EnchantmentKindId;
use crate::item::ItemKindId;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct SharedEnchantmentCompatibilityMap(
    pub HashMap<ItemKindId, EditionShared<Vec<EnchantmentKindId>>>,
);

impl CloneByEdition<EnchantmentCompatibilityMap> for SharedEnchantmentCompatibilityMap {
    fn clone_by_edition(&self, edition: Edition) -> EnchantmentCompatibilityMap {
        EnchantmentCompatibilityMap(self.contents_by_edition(edition))
    }
}

impl SharedEnchantmentCompatibilityMap {
    fn contents_by_edition(&self, edition: Edition) -> HashMap<ItemKindId, Vec<EnchantmentKindId>> {
        self.0.iter().map(clone_by_edition(edition)).collect()
    }
}

type SharedCompatibleItemEnchantments<'a> =
    (&'a ItemKindId, &'a EditionShared<Vec<EnchantmentKindId>>);

type CompatibleItemEnchantments = (ItemKindId, Vec<EnchantmentKindId>);

fn clone_by_edition(
    edition: Edition,
) -> impl FnMut(SharedCompatibleItemEnchantments) -> CompatibleItemEnchantments {
    move |(enchantment, incompatible)| (enchantment.clone(), incompatible.clone_by_edition(edition))
}
