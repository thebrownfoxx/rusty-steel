use crate::enchantment::cost_multiplier::CostMultiplier;
use crate::enchantment::Enchantment;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug)]
pub struct EnchantmentKindId(pub String);

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
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

    fn of_level(&self, level: u8) -> Option<Enchantment> {
        Enchantment::new(self, level)
    }
}
