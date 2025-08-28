pub mod agnostic_enchanter;
pub mod compatible_item_kind_enchanter;
pub mod compatible_enchantments_enchanter;

use crate::enchantment::Enchantment;
use crate::item::enchanter::compatible_item_kind_enchanter::IntoCompatibleItemKindEnchanter;
use crate::item::Item;

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash, Debug)]
pub enum EnchantmentError {
    ItemKindIncompatible,
    EnchantmentsIncompatible,
}

pub trait Enchanter {
    fn enchant(
        &self,
        item: &mut Item,
        enchantment: Enchantment,
    ) -> Result<(), EnchantmentError>;
}

fn x() {
}
