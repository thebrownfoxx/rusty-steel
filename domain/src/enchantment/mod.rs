mod cap_max_level;
mod cost_multiplier;
mod enchantment_kind;
mod enchantment_kinds;
mod max_level;
mod shared_enchantment_kind;
mod shared_enchantment_kinds;

pub use cap_max_level::{CapMaxLevel, MaxLevelCapMaxLevel};
pub use cost_multiplier::CostMultiplier;
pub use enchantment_kind::{EnchantmentKind, EnchantmentKindId};
pub use enchantment_kinds::{EnchantmentKinds, OwnedEnchantmentKinds};
pub use max_level::{EnchantmentKindsMaxLevel, MaxLevel};
pub use shared_enchantment_kind::SharedEnchantmentKind;
pub use shared_enchantment_kinds::SharedEnchantmentKinds;

use bon::Builder;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Eq, PartialEq, Clone, Hash, Debug, Serialize, Deserialize, Builder)]
pub struct Enchantment {
    #[builder(into)]
    pub kind: EnchantmentKindId,

    pub level: u8,
}

impl AsRef<EnchantmentKindId> for Enchantment {
    fn as_ref(&self) -> &EnchantmentKindId {
        &self.kind
    }
}
