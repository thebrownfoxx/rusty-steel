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
use bon::Builder;

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
