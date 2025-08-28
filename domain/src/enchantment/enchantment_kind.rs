use crate::enchantment::cost_multiplier::CostMultiplier;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, Serialize, Deserialize)]
pub struct EnchantmentKindId(pub String);

impl From<String> for EnchantmentKindId {
    fn from(value: String) -> Self {
        EnchantmentKindId(value)
    }
}

#[derive(Eq, PartialEq, Clone, Hash, Debug, Serialize, Deserialize)]
pub struct EnchantmentKind {
    pub id: EnchantmentKindId,
    pub name: String,
    pub max_level: u8,
    pub cost_multiplier: CostMultiplier,
}

impl EnchantmentKind {
    pub fn new(
        id: impl Into<EnchantmentKindId>,
        name: impl Into<String>,
        max_level: u8,
        cost_multiplier: CostMultiplier,
    ) -> Self {
        EnchantmentKind {
            id: id.into(),
            name: name.into(),
            max_level,
            cost_multiplier,
        }
    }
}
