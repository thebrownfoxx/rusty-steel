#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub struct EnchantmentTypeId(pub String);

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub struct CostMultiplier {
    pub item: i8,
    pub book: i8,
}

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub struct EnchantmentType {
    pub id: EnchantmentTypeId,
    pub name: String,
    pub max_level: i8,
    pub cost_multiplier: CostMultiplier,
}

pub trait EnchantmentTypes {
    fn all(&self) -> Vec<&EnchantmentTypeId>;
    fn get(&self, id: &EnchantmentTypeId) -> Option<&EnchantmentType>;
}

impl EnchantmentTypes for Vec<EnchantmentType> {
    fn all(&self) -> Vec<&EnchantmentTypeId> {
        self.iter().map(|enchantment_type| &enchantment_type.id).collect()
    }

    fn get(&self, id: &EnchantmentTypeId) -> Option<&EnchantmentType> {
        self.iter().find(|enchantment_type| enchantment_type.id == *id)
    }
}
