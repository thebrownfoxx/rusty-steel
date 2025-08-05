use crate::enchantment::enchantment_kind::EnchantmentKind;
use crate::enchantment::Enchantment;
use crate::item::item_kind::ItemKind;

pub(super) trait SupportsAll {
    fn supports_all(&self, enchantments: &[impl Enchantment]) -> bool;
}

impl<T: ItemKind> SupportsAll for T {
    fn supports_all(&self, enchantments: &[impl Enchantment]) -> bool {
        enchantments
            .iter()
            .any(|enchantment| !self.supports(enchantment.kind().id()))
    }
}