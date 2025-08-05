use crate::enchantment::cost_multiplier::CostMultiplier;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug)]
pub struct EnchantmentTypeId(pub String);

pub trait EnchantmentType: Eq + Clone + Hash + Debug {
    fn id(&self) -> &EnchantmentTypeId;
    fn name(&self) -> &str;
    fn max_level(&self) -> u8;
    fn cost_multiplier(&self) -> impl CostMultiplier;
}

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub struct OwnedEnchantmentType<C: CostMultiplier> {
    pub id: EnchantmentTypeId,
    pub name: String,
    pub max_level: u8,
    pub cost_multiplier: C,
}

impl<C: CostMultiplier> OwnedEnchantmentType<C> {
    pub fn new(
        id: impl Into<EnchantmentTypeId>,
        name: impl Into<String>,
        max_level: u8,
        cost_multiplier: C,
    ) -> Self {
        OwnedEnchantmentType {
            id: id.into(),
            name: name.into(),
            max_level,
            cost_multiplier,
        }
    }
}

impl<C: CostMultiplier> EnchantmentType for OwnedEnchantmentType<C> {
    fn id(&self) -> &EnchantmentTypeId {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn max_level(&self) -> u8 {
        self.max_level
    }

    fn cost_multiplier(&self) -> impl CostMultiplier {
        self.cost_multiplier
    }
}
