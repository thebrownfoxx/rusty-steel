use crate::enchantment::Enchantment;
use crate::item::item_builder::ItemBuilder;
use crate::item::item_kind::{ItemKind, ItemKindId};
use crate::item::item_kind_provider::ItemKindProvider;

pub mod item_builder;
pub mod item_kind;
pub mod item_kind_provider;
pub mod shared_item_kind;
pub mod shared_item_kind_provider;
mod supports_all;

pub struct Item {
    kind: ItemKindId,
    enchantments: Vec<Enchantment>,
    pub anvil_use_count: u8,
}

pub enum IntoItemBuilderError {
    KindNotFound,
    IncompatibleEnchantments,
}

impl Item {
    pub fn new(
        kind: &ItemKind,
        enchantments: Vec<Enchantment>,
        anvil_use_count: u8,
    ) -> Option<Self> {
        Some(ItemBuilder::with(kind, enchantments, anvil_use_count)?.build())
    }

    pub fn into_builder(
        self,
        kind_provider: &impl ItemKindProvider,
    ) -> Result<ItemBuilder, IntoItemBuilderError> {
        match kind_provider.get(&self.kind) {
            None => Err(IntoItemBuilderError::IncompatibleEnchantments),
            Some(kind) => match ItemBuilder::with(kind, self.enchantments, self.anvil_use_count) {
                None => Err(IntoItemBuilderError::IncompatibleEnchantments),
                Some(builder) => Ok(builder),
            },
        }
    }
}
