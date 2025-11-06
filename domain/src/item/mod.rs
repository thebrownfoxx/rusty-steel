mod enchant;
mod kind;
mod kinds;

pub use kind::{ItemKind, ItemKindId};
pub use kinds::{ItemKinds, OwnedItemKinds};
use std::collections::HashMap;

use crate::enchantment::{Enchantment, EnchantmentKindId, EnchantmentLevel};
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Item {
    pub kind: ItemKindId,
    pub enchantment_levels: HashMap<EnchantmentKindId, EnchantmentLevel>,
    pub anvil_use_count: u8,
}

impl Item {
    pub fn new(kind: impl Into<ItemKindId>) -> Self {
        Self {
            kind: kind.into(),
            enchantment_levels: HashMap::new(),
            anvil_use_count: 0,
        }
    }

    pub fn enchantment_kinds(&self) -> impl Iterator<Item = &EnchantmentKindId> {
        self.enchantment_levels.iter().map(|kind| kind)
    }

    pub fn enchantments(&self) -> impl Iterator<Item = Enchantment> {
        self.enchantment_levels
            .iter()
            .map(|(kind, level)| Enchantment::new(kind, level))
    }
}

impl AsRef<ItemKindId> for Item {
    fn as_ref(&self) -> &ItemKindId {
        &self.kind
    }
}
