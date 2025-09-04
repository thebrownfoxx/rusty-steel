use crate::enchantment::enchantment_kind::EnchantmentKindId;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::hash::Hash;

pub mod compatible;
pub mod cost_multiplier;
pub mod enchantment_kind;
pub mod enchantment_kinds;
pub mod shared_enchantment_kind;
pub mod shared_enchantment_kinds;

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
