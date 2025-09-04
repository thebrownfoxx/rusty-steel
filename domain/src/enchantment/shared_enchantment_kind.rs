use super::{CostMultiplier, EnchantmentKind, EnchantmentKindId};
use crate::edition::{CloneByEdition, Edition, EditionShared};
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Clone, Hash, Debug, Serialize, Deserialize)]
pub struct SharedEnchantmentKind {
    pub id: EnchantmentKindId,
    pub name: EditionShared<String>,
    pub max_level: EditionShared<u8>,
    pub cost_multiplier: EditionShared<CostMultiplier>,
}

impl CloneByEdition<EnchantmentKind> for SharedEnchantmentKind {
    fn clone_by_edition(&self, edition: Edition) -> EnchantmentKind {
        EnchantmentKind {
            id: self.id,
            name: self.name.clone_by_edition(edition),
            max_level: self.max_level.clone_by_edition(edition),
            cost_multiplier: self.cost_multiplier.clone_by_edition(edition),
        }
    }
}
