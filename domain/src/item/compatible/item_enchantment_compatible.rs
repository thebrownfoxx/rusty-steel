use crate::enchantment::enchantment_kind::EnchantmentKindId;
use crate::item::item_kind::ItemKindId;

pub trait ItemEnchantmentCompatible {
    fn are_compatible(&self, item: ItemKindId, enchantment: EnchantmentKindId) -> bool;
}
