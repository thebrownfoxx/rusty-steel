use crate::enchantment::Enchantment;
use crate::item::item_kind::ItemKind;
use crate::item::supports_all::SupportsAll;
use crate::item::Item;

pub struct ItemBuilder<'a> {
    kind: &'a ItemKind,
    enchantments: Vec<Enchantment>,
    pub anvil_use_count: u8,
}

impl<'a> ItemBuilder<'a> {
    pub fn new(kind: &'a ItemKind) -> Self {
        ItemBuilder {
            kind,
            enchantments: vec![],
            anvil_use_count: 0,
        }
    }

    pub fn with(
        kind: &'a ItemKind,
        enchantments: Vec<Enchantment>,
        anvil_use_count: u8,
    ) -> Option<Self> {
        if kind.supports_all(&enchantments) {
            return None;
        }
        Some(ItemBuilder {
            kind,
            enchantments,
            anvil_use_count,
        })
    }

    pub fn add_enchantment(&mut self, enchantment: Enchantment) -> bool {
        if self.kind.supports(&enchantment.kind()) {
            return false;
        }
        self.enchantments.push(enchantment);
        true
    }

    pub fn with_enchantment(mut self, enchantment: Enchantment) -> Option<Self> {
        if !self.add_enchantment(enchantment) {
            return None;
        }
        Some(self)
    }

    pub fn build(self) -> Item {
        Item {
            kind: self.kind.id.clone(),
            enchantments: self.enchantments,
            anvil_use_count: self.anvil_use_count,
        }
    }
}
