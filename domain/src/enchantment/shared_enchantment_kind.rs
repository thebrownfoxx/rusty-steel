use serde::{Deserialize, Serialize};
use crate::edition::edition_shared::EditionShared;
use crate::edition::{CloneByEdition, Edition};
use crate::enchantment::cost_multiplier::CostMultiplier;
use crate::enchantment::enchantment_kind::{EnchantmentKind, EnchantmentKindId};

#[derive(Eq, PartialEq, Clone, Hash, Debug, Serialize, Deserialize)]
pub struct SharedEnchantmentKind {
    pub id: EnchantmentKindId,
    pub name: EditionShared<String>,
    pub max_level: EditionShared<u8>,
    pub cost_multiplier: EditionShared<CostMultiplier>,
}

impl SharedEnchantmentKind {
    pub fn new(
        id: impl Into<EnchantmentKindId>,
        name: impl Into<EditionShared<String>>,
        max_level: impl Into<EditionShared<u8>>,
        cost_multiplier: impl Into<EditionShared<CostMultiplier>>,
    ) -> SharedEnchantmentKind {
        SharedEnchantmentKind {
            id: id.into(),
            name: name.into(),
            max_level: max_level.into(),
            cost_multiplier: cost_multiplier.into(),
        }
    }
}

impl CloneByEdition<EnchantmentKind> for SharedEnchantmentKind {
    fn clone_by_edition(&self, edition: Edition) -> EnchantmentKind {
        EnchantmentKind::new(
            self.id.clone(),
            self.name.clone_by_edition(edition),
            self.max_level.clone_by_edition(edition),
            self.cost_multiplier.clone_by_edition(edition),
        )
    }
}
