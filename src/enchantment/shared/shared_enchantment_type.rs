use crate::enchantment::enchantment_type::{CostMultiplier, EnchantmentTypeId};
use SharedCostMultiplier as SharedMultiplier;

pub struct SharedEnchantmentType {
    pub id: EnchantmentTypeId,
    pub name: String,
    pub cost_multiplier: SharedCostMultiplier,
}

pub enum SharedCostMultiplier {
    JavaOnly(CostMultiplier),
    BedrockOnly(CostMultiplier),
    CrossPlatform {
        java: CostMultiplier,
        bedrock: CostMultiplier,
    },
}

impl SharedCostMultiplier {
    fn same(cost_multiplier: CostMultiplier) -> SharedCostMultiplier {
        SharedMultiplier::CrossPlatform {
            java: cost_multiplier,
            bedrock: cost_multiplier,
        }
    }

    fn java(&self) -> Option<&CostMultiplier> {
        match self {
            SharedMultiplier::JavaOnly(multiplier) => Some(multiplier),
            SharedMultiplier::BedrockOnly(_) => None,
            SharedMultiplier::CrossPlatform { java, .. } => Some(java),
        }
    }

    fn bedrock(&self) -> Option<&CostMultiplier> {
        match self {
            SharedMultiplier::JavaOnly(_) => None,
            SharedMultiplier::BedrockOnly(multiplier) => Some(multiplier),
            SharedMultiplier::CrossPlatform { bedrock, .. } => Some(bedrock),
        }
    }
}
