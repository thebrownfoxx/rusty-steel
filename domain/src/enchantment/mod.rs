use crate::enchantment::enchantment_kind::EnchantmentKindId;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::hash::Hash;

pub mod cost_multiplier;
pub mod enchantment_compatibility;
pub mod enchantment_kind;
pub mod enchantment_kind_provider;
pub mod shared_enchantment_incompatibility_matrix;
pub mod shared_enchantment_kind;
pub mod shared_enchantment_kind_provider;

#[derive(Eq, PartialEq, Clone, Hash, Debug, Serialize, Deserialize)]
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

    pub fn kind(&self) -> &EnchantmentKindId {
        &self.kind
    }

    pub fn level(&self) -> u8 {
        self.level
    }
}
