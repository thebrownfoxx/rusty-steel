use crate::enchantment::Enchantment;
use crate::enchantment::enchantment_kind::EnchantmentKind;
use crate::item::item_kind::ItemKind;
use crate::item::supports_all::SupportsAll;

pub mod item_kind;
pub mod item_kind_provider;
mod supports_all;

pub trait Item<E: Enchantment> {
    fn kind(&self) -> &impl ItemKind;

    fn enchantments<'a>(&'a self) -> impl Iterator<Item = &'a E>
    where
        E: 'a;

    fn anvil_use_count(&self) -> u8;

    fn add_enchantment(&mut self, enchantment: E) -> bool;
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

impl<T: ItemKind, E: Enchantment> Item<E> for OwnedItem<T, E> {
    fn kind(&self) -> &impl ItemKind {
        &self.kind
    }

    fn enchantments<'a>(&'a self) -> impl Iterator<Item = &'a E>
    where
        E: 'a,
    {
        self.enchantments.iter()
    }

    fn anvil_use_count(&self) -> u8 {
        self.anvil_use_count
    }

    fn add_enchantment(&mut self, enchantment: E) -> bool {
        if self.kind().supports(enchantment.kind().id()) {
            return false;
        }

        self.enchantments.push(enchantment);
        true
    }
}
