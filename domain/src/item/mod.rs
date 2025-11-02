mod kind;
mod kinds;

pub use kind::{ItemKind, ItemKindId};
pub use kinds::{ItemKinds, OwnedItemKinds};

use crate::enchantment::{Enchantment, EnchantmentKindId};
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Item {
    pub kind: ItemKindId,
    pub enchantments: Vec<Enchantment>,
    pub anvil_use_count: u8,
}

impl Item {
    pub fn new(kind: impl Into<ItemKindId>) -> Self {
        Self {
            kind: kind.into(),
            enchantments: vec![],
            anvil_use_count: 0,
        }
    }

    pub fn enchantment_kinds(&self) -> impl Iterator<Item = &EnchantmentKindId> {
        self.enchantments
            .iter()
            .map(|enchantment| &enchantment.kind)
    }
}

impl AsRef<ItemKindId> for Item {
    fn as_ref(&self) -> &ItemKindId {
        &self.kind
    }
}
