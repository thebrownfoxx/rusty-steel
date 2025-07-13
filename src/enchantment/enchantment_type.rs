pub struct EnchantmentTypeId(pub String);

#[derive(Copy, Clone)]
pub struct CostMultiplier {
    pub item: i8,
    pub book: i8,
}

pub struct EnchantmentType {
    pub id: EnchantmentTypeId,
    pub name: String,
    pub cost_multiplier: CostMultiplier,
}

pub trait EnchantmentTypes<'a> {
    fn all() -> Vec<&'a EnchantmentTypeId>;
    fn get(id: &EnchantmentTypeId) -> &EnchantmentType;
}
