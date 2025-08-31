use crate::edition::edition_shared::EditionShared;
use crate::edition::{CloneByEdition, Edition};
use crate::enchantment::enchantment_kind::EnchantmentKindId;
use crate::item::item_enchantment_compatibility::ItemEnchantmentCompatibilityMatrix;
use crate::item::item_kind::ItemKindId;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct SharedItemEnchantmentCompatibilityMatrix(
    pub HashMap<ItemKindId, EditionShared<Vec<EnchantmentKindId>>>,
);

impl CloneByEdition<ItemEnchantmentCompatibilityMatrix>
    for SharedItemEnchantmentCompatibilityMatrix
{
    fn clone_by_edition(&self, edition: Edition) -> ItemEnchantmentCompatibilityMatrix {
        ItemEnchantmentCompatibilityMatrix(self.contents_by_edition(edition))
    }
}

impl SharedItemEnchantmentCompatibilityMatrix {
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
