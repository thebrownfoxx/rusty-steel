use crate::edition::edition_shared::EditionShared;
use crate::edition::{CloneByEdition, Edition};
use crate::enchantment::enchantment_compatibility::EnchantmentIncompatibilityMatrix;
use crate::enchantment::enchantment_kind::EnchantmentKindId;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct SharedEnchantmentIncompatibilityMatrix(
    pub HashMap<EnchantmentKindId, EditionShared<Vec<EnchantmentKindId>>>,
);

impl CloneByEdition<EnchantmentIncompatibilityMatrix> for SharedEnchantmentIncompatibilityMatrix {
    fn clone_by_edition(&self, edition: Edition) -> EnchantmentIncompatibilityMatrix {
        EnchantmentIncompatibilityMatrix(self.contents_by_edition(edition))
    }
}

impl SharedEnchantmentIncompatibilityMatrix {
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
    move |(enchantment, incompatible)| (*enchantment, incompatible.clone_by_edition(edition))
}
