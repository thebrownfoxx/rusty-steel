use crate::enchant::{AgnosticEnchant, Enchant, Error, Result};
use crate::enchantment::{Enchantment, EnchantmentKindId};
use crate::item::{Item, ItemEnchantmentCompatible, ItemKindId};
use std::rc::Rc;

pub struct CompatibleItemKindEnchant {
    pub enchanter: Box<dyn Enchant>,
    pub item_enchantment_compatibility: Rc<dyn ItemEnchantmentCompatible>,
}

impl CompatibleItemKindEnchant {
    pub fn wrap(
        enchanter: Box<dyn Enchant>,
        item_enchantment_compatibility: Rc<dyn ItemEnchantmentCompatible>,
    ) -> Self {
        Self {
            enchanter,
            item_enchantment_compatibility,
        }
    }

    pub fn new(item_enchantment_compatibility: Rc<dyn ItemEnchantmentCompatible>) -> Self {
        Self::wrap(Box::new(AgnosticEnchant), item_enchantment_compatibility)
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
            true => self.enchanter.enchant(item, enchantment),
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
        CompatibleItemKindEnchant::wrap(Box::new(self), item_enchantment_compatibility)
    }
}
