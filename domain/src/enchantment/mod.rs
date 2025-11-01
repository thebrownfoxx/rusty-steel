pub mod combine;

mod cost_multiplier;
mod kind;
mod kinds;
mod level;

pub use cost_multiplier::CostMultiplier;
pub use kind::{EnchantmentKind, EnchantmentKindId};
pub use kinds::{EnchantmentKinds, OwnedEnchantmentKinds};

use crate::enchantment::level::EnchantmentLevel;
use bon::Builder;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Eq, PartialEq, Clone, Hash, Debug, Serialize, Deserialize, Builder)]
pub struct Enchantment {
    #[builder(into)]
    pub kind: EnchantmentKindId,

    pub level: EnchantmentLevel,
}

impl AsRef<EnchantmentKindId> for Enchantment {
    fn as_ref(&self) -> &EnchantmentKindId {
        &self.kind
    }
}
