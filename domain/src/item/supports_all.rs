use crate::enchantment::Enchantment;
use crate::item::item_kind::ItemKind;

pub(super) trait SupportsAll {
    fn supports_all(&self, enchantments: &[Enchantment]) -> bool;
}

impl SupportsAll for ItemKind {
    fn supports_all(&self, enchantments: &[Enchantment]) -> bool {
        enchantments
            .iter()
            .any(|enchantment| !self.supports(enchantment.kind()))
    }
}
