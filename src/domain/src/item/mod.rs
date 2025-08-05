use crate::enchantment::Enchantment;
use crate::item::item_type::ItemType;

pub mod item_type;
pub mod item_type_provider;

pub trait Item {
    fn get_type(&self) -> &impl ItemType;
    fn enchantments(&self) -> impl Iterator<Item = &impl Enchantment>;
    fn anvil_use_count(&self) -> u8;
}
