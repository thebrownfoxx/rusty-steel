use crate::enchantment::enchantment_kind::EnchantmentKindId;
use crate::enchantment::Enchantment;
use crate::item::enchanter::agnostic_enchanter::AgnosticEnchanter;
use crate::item::enchanter::{Enchanter, EnchantmentError};
use crate::item::item_enchantment_compatibility::ItemEnchantmentCompatibility;
use crate::item::item_kind::ItemKindId;
use crate::item::Item;
use std::rc::Rc;

#[derive(Clone)]
pub struct CompatibleItemKindEnchanter {
    pub enchanter: Rc<dyn Enchanter>,
    pub item_enchantment_compatibility: Rc<dyn ItemEnchantmentCompatibility>,
}

impl CompatibleItemKindEnchanter {
    pub fn wrap(
        enchanter: Rc<dyn Enchanter>,
        item_enchantment_compatibility: Rc<dyn ItemEnchantmentCompatibility>,
    ) -> Self {
        Self {
            enchanter,
            item_enchantment_compatibility,
        }
    }

    pub fn new(item_enchantment_compatibility: Rc<dyn ItemEnchantmentCompatibility>) -> Self {
        Self::wrap(Rc::new(AgnosticEnchanter), item_enchantment_compatibility)
    }

    fn are_compatible(&self, item: ItemKindId, enchantment: EnchantmentKindId) -> bool {
        self.item_enchantment_compatibility
            .are_compatible(item, enchantment)
    }
}

impl Enchanter for CompatibleItemKindEnchanter {
    fn enchant(&self, item: &mut Item, enchantment: Enchantment) -> Result<(), EnchantmentError> {
        match self.are_compatible(item.kind, enchantment.kind) {
            true => self.enchanter.enchant(item, enchantment),
            false => Err(EnchantmentError::ItemKindIncompatible),
        }
    }
}

pub trait RequireCompatibleItemKind {
    fn require_compatible_item_kind(
        self,
        item_enchantment_compatibility: Rc<dyn ItemEnchantmentCompatibility>,
    ) -> CompatibleItemKindEnchanter
    where
        Self: Sized;
}

impl RequireCompatibleItemKind for Rc<dyn Enchanter> {
    fn require_compatible_item_kind(
        self,
        item_enchantment_compatibility: Rc<dyn ItemEnchantmentCompatibility>,
    ) -> CompatibleItemKindEnchanter
    where
        Self: Sized,
    {
        CompatibleItemKindEnchanter::wrap(self, item_enchantment_compatibility)
    }
}
