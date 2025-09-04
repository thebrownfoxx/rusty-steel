mod compatible;
mod cost_multiplier;
mod enchantment_kind;
mod enchantment_kinds;
mod shared_enchantment_kind;
mod shared_enchantment_kinds;

pub use compatible::{
    EnchantmentIncompatibilityMatrix, EnchantmentsCompatible,
    SharedEnchantmentIncompatibilityMatrix,
};
pub use cost_multiplier::CostMultiplier;
pub use enchantment_kind::{EnchantmentKind, EnchantmentKindId};
pub use enchantment_kinds::{EnchantmentKinds, OwnedEnchantmentKinds};
pub use shared_enchantment_kind::SharedEnchantmentKind;
pub use shared_enchantment_kinds::SharedEnchantmentKinds;

use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug, Serialize, Deserialize)]
pub struct Enchantment {
    pub kind: EnchantmentKindId,
    pub level: u8,
}

impl Enchantment {
    pub fn new(kind: impl Into<EnchantmentKindId>, level: u8) -> Self {
        Self {
            kind: kind.into(),
            level,
        }
    }
}
