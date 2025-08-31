use crate::enchantment::Enchantment;
use crate::item::enchanter::{Enchanter, EnchantmentError};
use crate::item::Item;

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
pub struct AgnosticEnchanter;

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
