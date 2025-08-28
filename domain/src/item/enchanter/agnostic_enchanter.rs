use crate::enchantment::Enchantment;
use crate::item::Item;
use crate::item::enchanter::{Enchanter, EnchantmentError};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct AgnosticEnchanter;

impl AgnosticEnchanter {
    pub fn new() -> Self {
        AgnosticEnchanter
    }
}

impl Enchanter for AgnosticEnchanter {
    fn enchant(
        &self,
        item: &mut Item,
        enchantment: Enchantment,
    ) -> Result<(), EnchantmentError> {
        item.enchantments.push(enchantment);
        Ok(())
    }
}
