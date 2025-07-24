use crate::enchantment::cost_multiplier::CostMultiplier;
use crate::enchantment::enchantment_type_id::EnchantmentTypeId;

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub struct EnchantmentType {
    pub id: EnchantmentTypeId,
    pub name: String,
    pub max_level: i8,
    pub cost_multiplier: CostMultiplier,
}

impl EnchantmentType {
    pub fn new(
        id: impl Into<EnchantmentTypeId>,
        name: impl Into<String>,
        max_level: i8,
        cost_multiplier: CostMultiplier
    ) -> EnchantmentType {
        EnchantmentType {
            id: id.into(),
            name: name.into(),
            max_level,
            cost_multiplier
        }
    }
}
