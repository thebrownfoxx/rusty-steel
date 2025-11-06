mod combine;
mod cost_multiplier;
mod kind;
mod kinds;
mod level;

pub use crate::enchantment::level::EnchantmentLevel;
pub use combine::CombineEnchantments;
pub use cost_multiplier::CostMultiplier;
pub use kind::{EnchantmentKind, EnchantmentKindId};
pub use kinds::{EnchantmentKinds, OwnedEnchantmentKinds};

use std::fmt::Debug;
use std::hash::Hash;

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub struct Enchantment {
    pub kind: EnchantmentKindId,
    pub level: EnchantmentLevel,
}

impl Enchantment {
    pub fn new(kind: impl Into<EnchantmentKindId>, level: impl Into<EnchantmentLevel>) -> Self {
        Self {
            kind: kind.into(),
            level: level.into(),
        }
    }
}

impl AsRef<EnchantmentKindId> for Enchantment {
    fn as_ref(&self) -> &EnchantmentKindId {
        &self.kind
    }
}
