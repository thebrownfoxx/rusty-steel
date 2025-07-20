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
    pub cost_multiplier: CostMultiplier,
}

pub trait EnchantmentTypes {
    fn all(&self) -> Vec<EnchantmentTypeId>;
    fn get(&self, id: &EnchantmentTypeId) -> Option<EnchantmentType>;
}
