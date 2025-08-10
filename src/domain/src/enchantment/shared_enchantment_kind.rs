use crate::edition::edition_shared::EditionShared;
use crate::edition::{CloneByEdition, Edition};
use crate::enchantment::cost_multiplier::CostMultiplier;
use crate::enchantment::enchantment_kind::{EnchantmentKind, EnchantmentKindId};

pub struct SharedEnchantmentKind {
    pub id: EnchantmentKindId,
    pub name: EditionShared<String>,
    pub max_level: EditionShared<u8>,
    pub cost_multiplier: EditionShared<CostMultiplier>,
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
