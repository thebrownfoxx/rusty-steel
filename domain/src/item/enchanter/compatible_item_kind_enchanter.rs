use crate::enchantment::Enchantment;
use crate::enchantment::enchantment_kind::EnchantmentKindId;
use crate::item::Item;
use crate::item::enchanter::agnostic_enchanter::AgnosticEnchanter;
use crate::item::enchanter::{Enchanter, EnchantmentError};
use crate::item::item_enchantment_compatibility::ItemEnchantmentCompatibility;
use crate::item::item_kind::ItemKindId;
use std::sync::Arc;

#[derive(Clone)]
pub struct CompatibleItemKindEnchanter {
    pub enchanter: Arc<dyn Enchanter>,
    pub item_enchantment_compatibility: Arc<dyn ItemEnchantmentCompatibility>,
}

impl CompatibleItemKindEnchanter {
    pub fn new(
        item_enchantment_compatibility: impl Into<Arc<dyn ItemEnchantmentCompatibility>>,
    ) -> Self {
        Self {
            enchanter: Arc::new(AgnosticEnchanter),
            item_enchantment_compatibility: item_enchantment_compatibility.into(),
        }
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

pub trait IntoCompatibleItemKindEnchanter {
    fn into_compatible_item_kind_enchanter(
        self,
        item_enchantment_compatibility: impl Into<Arc<dyn ItemEnchantmentCompatibility>>,
    ) -> CompatibleItemKindEnchanter
    where
        Self: Sized;
}

impl<T> IntoCompatibleItemKindEnchanter for T
where
    T: Into<Arc<dyn Enchanter>>,
{
    fn into_compatible_item_kind_enchanter(
        self,
        item_enchantment_compatibility: impl Into<Arc<dyn ItemEnchantmentCompatibility>>,
    ) -> CompatibleItemKindEnchanter
    where
        Self: Sized,
    {
        CompatibleItemKindEnchanter {
            enchanter: self.into(),
            item_enchantment_compatibility: item_enchantment_compatibility.into(),
        }
    }
}
