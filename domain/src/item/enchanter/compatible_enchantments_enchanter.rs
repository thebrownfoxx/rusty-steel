use crate::enchantment::enchantment_compatibility::EnchantmentCompatibility;
use crate::enchantment::enchantment_kind::EnchantmentKindId;
use crate::enchantment::Enchantment;
use crate::item::enchanter::agnostic_enchanter::AgnosticEnchanter;
use crate::item::enchanter::{Enchanter, EnchantmentError};
use crate::item::Item;
use std::rc::Rc;

#[derive(Clone)]
pub struct CompatibleEnchantmentsEnchanter {
    pub enchanter: Rc<dyn Enchanter>,
    pub enchantment_compatibility: Rc<dyn EnchantmentCompatibility>,
}

impl CompatibleEnchantmentsEnchanter {
    pub fn wrap(
        enchanter: Rc<dyn Enchanter>,
        enchantment_compatibility: Rc<dyn EnchantmentCompatibility>,
    ) -> Self {
        Self {
            enchanter,
            enchantment_compatibility,
        }
    }

    pub fn new(enchantment_compatibility: Rc<dyn EnchantmentCompatibility>) -> Self {
        Self::wrap(Rc::new(AgnosticEnchanter), enchantment_compatibility)
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

pub trait RequireCompatibleEnchantments {
    fn require_compatible_enchantments(
        self,
        enchantment_compatibility: Rc<dyn EnchantmentCompatibility>,
    ) -> CompatibleEnchantmentsEnchanter
    where
        Self: Sized;
}

impl RequireCompatibleEnchantments for Rc<dyn Enchanter> {
    fn require_compatible_enchantments(
        self,
        enchantment_compatibility: Rc<dyn EnchantmentCompatibility>,
    ) -> CompatibleEnchantmentsEnchanter
    where
        Self: Sized,
    {
        CompatibleEnchantmentsEnchanter::wrap(self, enchantment_compatibility)
    }
}
