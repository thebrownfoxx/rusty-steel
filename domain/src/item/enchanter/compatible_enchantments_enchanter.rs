use crate::enchantment::Enchantment;
use crate::enchantment::enchantment_compatibility::EnchantmentCompatibility;
use crate::enchantment::enchantment_kind::EnchantmentKindId;
use crate::item::Item;
use crate::item::enchanter::agnostic_enchanter::AgnosticEnchanter;
use crate::item::enchanter::{Enchanter, EnchantmentError};
use std::sync::Arc;

#[derive(Clone)]
pub struct CompatibleEnchantmentsEnchanter {
    pub enchanter: Arc<dyn Enchanter>,
    pub enchantment_compatibility: Arc<dyn EnchantmentCompatibility>,
}

impl CompatibleEnchantmentsEnchanter {
    pub fn new(enchantment_compatibility: impl Into<Arc<dyn EnchantmentCompatibility>>) -> Self {
        Self {
            enchanter: Arc::new(AgnosticEnchanter),
            enchantment_compatibility: enchantment_compatibility.into(),
        }
    }

    fn are_compatible(
        &self,
        enchantment_a: EnchantmentKindId,
        enchantment_b: EnchantmentKindId,
    ) -> bool {
        self.enchantment_compatibility
            .are_compatible(enchantment_a, enchantment_b)
    }

    fn new_enchantment_compatible(
        &self,
        mut existing_enchantments: impl Iterator<Item = EnchantmentKindId>,
        new_enchantment: EnchantmentKindId,
    ) -> bool {
        existing_enchantments.all(|existing| self.are_compatible(existing, new_enchantment))
    }
}

impl Enchanter for CompatibleEnchantmentsEnchanter {
    fn enchant(&self, item: &mut Item, enchantment: Enchantment) -> Result<(), EnchantmentError> {
        match self.new_enchantment_compatible(item.enchantment_kinds(), enchantment.kind) {
            true => self.enchanter.enchant(item, enchantment),
            false => Err(EnchantmentError::EnchantmentsIncompatible),
        }
    }
}

pub trait IntoCompatibleEnchantmentsEnchanter {
    fn into_compatible_item_kind_enchanter(
        self,
        enchantment_compatibility: impl Into<Arc<dyn EnchantmentCompatibility>>,
    ) -> CompatibleEnchantmentsEnchanter
    where
        Self: Sized;
}

impl<T> IntoCompatibleEnchantmentsEnchanter for T
where
    T: Into<Arc<dyn Enchanter>>,
{
    fn into_compatible_item_kind_enchanter(
        self,
        enchantment_compatibility: impl Into<Arc<dyn EnchantmentCompatibility>>,
    ) -> CompatibleEnchantmentsEnchanter
    where
        Self: Sized,
    {
        CompatibleEnchantmentsEnchanter {
            enchanter: self.into(),
            enchantment_compatibility: enchantment_compatibility.into(),
        }
    }
}
