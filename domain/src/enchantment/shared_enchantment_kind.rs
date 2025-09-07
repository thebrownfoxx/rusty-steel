use super::{CostMultiplier, EnchantmentKind, EnchantmentKindId};
use crate::edition::{CloneByEdition, Edition, EditionShared};
use bon::Builder;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Eq, PartialEq, Clone, Hash, Debug, Serialize, Deserialize, Builder)]
pub struct SharedEnchantmentKind {
    #[builder(into)]
    pub id: EnchantmentKindId,
    #[builder(into)]
    pub name: EditionShared<Rc<str>>,
    #[builder(into)]
    pub max_level: EditionShared<u8>,
    #[builder(into)]
    pub cost_multiplier: EditionShared<CostMultiplier>,
}

impl CloneByEdition<EnchantmentKind> for SharedEnchantmentKind {
    fn clone_by_edition(&self, edition: Edition) -> EnchantmentKind {
        EnchantmentKind {
            id: self.id.clone(),
            name: self.name.clone_by_edition(edition),
            max_level: self.max_level.clone_by_edition(edition),
            cost_multiplier: self.cost_multiplier.clone_by_edition(edition),
        }
    }
}

impl From<&SharedEnchantmentKind> for EnchantmentKindId {
    fn from(value: &SharedEnchantmentKind) -> Self {
        value.id.clone()
    }
}
