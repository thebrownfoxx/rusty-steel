use std::borrow::Cow;

#[derive(PartialEq)]
pub struct EnchantmentTypeId(pub String);

#[derive(Copy, Clone)]
pub struct CostMultiplier {
    pub item: i8,
    pub book: i8,
}

pub struct EnchantmentType<'a> {
    pub id: &'a EnchantmentTypeId,
    name: Cow<'a, str>,
    pub cost_multiplier: CostMultiplier,
}

impl<'a> EnchantmentType<'a> {
    pub fn new(
        id: &'a EnchantmentTypeId,
        name: Cow<'a, str>,
        cost_multiplier: CostMultiplier,
    ) -> Self {
        EnchantmentType {
            id,
            name,
            cost_multiplier,
        }
    }

    fn name(&self) -> &str {
        &self.name
    }
}

pub trait EnchantmentTypes<'a> {
    fn all(&self) -> Vec<&EnchantmentTypeId>;
    fn get(&self, id: &EnchantmentTypeId) -> Option<EnchantmentType>;
}
