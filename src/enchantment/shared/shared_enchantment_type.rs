use crate::enchantment::enchantment_type::{CostMultiplier, EnchantmentType, EnchantmentTypeId};
use std::sync::Arc;
use SharedCostMultiplier as SharedMultiplier;

pub struct SharedEnchantmentType<'a> {
    pub id: &'a EnchantmentTypeId,
    pub name: Arc<str>,
    pub cost_multiplier: SharedCostMultiplier,
}

impl SharedEnchantmentType<'_> {
    fn java(&self) -> Option<EnchantmentType> {
        Some(EnchantmentType {
            id: self.id,
            name: self.name.clone(),
            cost_multiplier: self.cost_multiplier.java()?,
        })
    }

    fn bedrock(&self) -> Option<EnchantmentType> {
        Some(EnchantmentType {
            id: self.id,
            name: self.name.clone(),
            cost_multiplier: self.cost_multiplier.bedrock()?,
        })
    }
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

    fn java(&self) -> Option<CostMultiplier> {
        match *self {
            SharedMultiplier::JavaOnly(multiplier) => Some(multiplier),
            SharedMultiplier::BedrockOnly(_) => None,
            SharedMultiplier::CrossPlatform { java, .. } => Some(java),
        }
    }

    fn bedrock(&self) -> Option<CostMultiplier> {
        match self {
            SharedMultiplier::JavaOnly(_) => None,
            SharedMultiplier::BedrockOnly(multiplier) => Some(*multiplier),
            SharedMultiplier::CrossPlatform { bedrock, .. } => Some(*bedrock),
        }
    }
}
