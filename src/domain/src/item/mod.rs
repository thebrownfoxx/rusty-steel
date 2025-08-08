use crate::enchantment::enchantment_kind::EnchantmentKind;
use crate::enchantment::Enchantment;
use crate::item::item_kind::ItemKind;
use crate::item::supports_all::SupportsAll;

pub mod item_kind;
pub mod item_kind_provider;
mod supports_all;

pub trait Item {
    fn kind(&self) -> &impl ItemKind;
    fn enchantments(&self) -> impl Iterator<Item = &impl Enchantment>;
    fn anvil_use_count(&self) -> u8;

    fn add_enchantment(&mut self, enchantment: impl Enchantment) -> bool;
}

pub struct OwnedItem<T: ItemKind, E: Enchantment> {
    pub kind: T,
    enchantments: Vec<E>,
    pub anvil_use_count: u8,
}

impl<T: ItemKind, E: Enchantment> OwnedItem<T, E> {
    pub fn new(kind: T, enchantments: Vec<E>, anvil_use_count: u8) -> Option<Self> {
        if kind.supports_all(&enchantments) {
            return None;
        }
        Some(OwnedItem {
            kind,
            enchantments,
            anvil_use_count,
        })
    }
}

impl<T: ItemKind, E: Enchantment> Item for OwnedItem<T, E> {
    fn kind(&self) -> &impl ItemKind {
        &self.kind
    }

    fn enchantments(&self) -> impl Iterator<Item = &impl Enchantment> {
        self.enchantments.iter()
    }

    fn anvil_use_count(&self) -> u8 {
        self.anvil_use_count
    }

    fn add_enchantment(&mut self, enchantment: impl Enchantment) -> bool {
        if self.kind().supports(enchantment.kind().id()) {
            return false;
        }

        match E::convert(&enchantment) {
            None => false,
            Some(converted) => {
                self.enchantments.push(converted);
                true
            }
        }
    }
}
