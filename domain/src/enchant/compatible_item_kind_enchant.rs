use crate::enchant::{AgnosticEnchant, Enchant, Error, Result};
use crate::enchantment::{Enchantment, EnchantmentKindId};
use crate::item::{Item, ItemEnchantmentCompatible, ItemKindId};
use std::rc::Rc;

pub struct CompatibleItemKindEnchant {
    pub enchant: Box<dyn Enchant>,
    pub item_enchantment_compatibility: Rc<dyn ItemEnchantmentCompatible>,
}

impl CompatibleItemKindEnchant {
    pub fn new(item_enchantment_compatibility: Rc<dyn ItemEnchantmentCompatible>) -> Self {
        AgnosticEnchant.require_compatible_item_kind(item_enchantment_compatibility)
    }

    fn are_compatible(
        &self,
        item: &dyn AsRef<ItemKindId>,
        enchantment: &dyn AsRef<EnchantmentKindId>,
    ) -> bool {
        self.item_enchantment_compatibility
            .are_compatible(item, enchantment)
    }
}

impl Enchant for CompatibleItemKindEnchant {
    fn enchant(&self, item: &mut Item, enchantment: Enchantment) -> Result<()> {
        match self.are_compatible(&item.kind, &enchantment.kind) {
            true => self.enchant.enchant(item, enchantment),
            false => Err(Error::ItemKindIncompatible),
        }
    }
}

pub trait RequireCompatibleItemKind {
    fn require_compatible_item_kind(
        self,
        item_enchantment_compatibility: Rc<dyn ItemEnchantmentCompatible>,
    ) -> CompatibleItemKindEnchant;
}

impl<T> RequireCompatibleItemKind for T
where
    T: Enchant + 'static,
{
    fn require_compatible_item_kind(
        self,
        item_enchantment_compatibility: Rc<dyn ItemEnchantmentCompatible>,
    ) -> CompatibleItemKindEnchant {
        CompatibleItemKindEnchant {
            enchant: Box::new(self),
            item_enchantment_compatibility,
        }
    }
}
