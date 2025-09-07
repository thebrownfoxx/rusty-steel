use super::EnchantmentIncompatibilityMatrix;
use crate::edition::{CloneByEdition, Edition, EditionShared};
use crate::enchantment::EnchantmentKindId;
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
    move |(enchantment, incompatible)| (enchantment.clone(), incompatible.clone_by_edition(edition))
}
