use crate::enchantment::cost_multiplier::CostMultiplier;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug)]
pub struct EnchantmentKindId(pub String);

pub trait EnchantmentKind: Eq + Clone + Hash + Debug {
    fn id(&self) -> &EnchantmentKindId;
    fn name(&self) -> &str;
    fn max_level(&self) -> u8;
    fn cost_multiplier(&self) -> impl CostMultiplier;
}

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub struct OwnedEnchantmentKind<C: CostMultiplier> {
    pub id: EnchantmentKindId,
    pub name: String,
    pub max_level: u8,
    pub cost_multiplier: C,
}

impl<C: CostMultiplier> OwnedEnchantmentKind<C> {
    pub fn new(
        id: impl Into<EnchantmentKindId>,
        name: impl Into<String>,
        max_level: u8,
        cost_multiplier: C,
    ) -> Self {
        OwnedEnchantmentKind {
            id: id.into(),
            name: name.into(),
            max_level,
            cost_multiplier,
        }
    }
}

impl<C: CostMultiplier> EnchantmentKind for OwnedEnchantmentKind<C> {
    fn id(&self) -> &EnchantmentKindId {
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
