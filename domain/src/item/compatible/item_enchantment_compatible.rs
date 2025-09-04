use crate::enchantment::EnchantmentKindId;
use crate::item::ItemKindId;

pub trait ItemEnchantmentCompatible {
    fn are_compatible(&self, item: ItemKindId, enchantment: EnchantmentKindId) -> bool;
}
