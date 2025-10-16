mod compatible;
mod item_kind;
mod item_kinds;
mod shared_item_kind;
mod shared_item_kinds;

use std::ops::Deref;
use bon::Builder;
pub use compatible::{
    EnchantmentCompatibilityMap, EnchantmentCompatible, SharedEnchantmentCompatibilityMap,
};
pub use item_kind::{ItemKind, ItemKindId};
pub use item_kinds::{ItemKinds, OwnedItemKinds};
pub use shared_item_kind::SharedItemKind;
pub use shared_item_kinds::SharedItemKinds;

use crate::enchantment::{Enchantment, EnchantmentKindId};
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, Builder)]
pub struct Item {
    #[builder(into)]
    pub kind: ItemKindId,

    #[builder(into)]
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
