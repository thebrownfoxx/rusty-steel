use crate::enchantment::Enchantment;
use crate::item::item_kind::ItemKind;

pub mod item_kind;
pub mod item_kind_provider;

pub trait Item {
    fn kind(&self) -> &impl ItemKind;
    fn enchantments(&self) -> impl Iterator<Item = &impl Enchantment>;
    fn anvil_use_count(&self) -> u8;
}
