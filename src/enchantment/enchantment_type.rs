use std::sync::Arc;

pub struct EnchantmentTypeId(pub String);

#[derive(Copy, Clone)]
pub struct CostMultiplier {
    pub item: i8,
    pub book: i8,
}

pub struct EnchantmentType<'a> {
    pub id: &'a EnchantmentTypeId,
    pub name: Arc<str>,
    pub cost_multiplier: CostMultiplier,
}

pub trait EnchantmentTypes<'a> {
    fn all() -> Vec<&'a EnchantmentTypeId>;
    fn get(id: &EnchantmentTypeId) -> &EnchantmentType;
}
