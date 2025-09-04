use super::agnostic_enchant::AgnosticEnchant;
use super::Result;
use super::{Enchant, Error};
use crate::enchantment::compatible::enchantments_compatible::EnchantmentsCompatible;
use crate::enchantment::enchantment_kind::EnchantmentKindId;
use crate::enchantment::Enchantment;
use crate::item::Item;
use std::rc::Rc;

#[derive(Clone)]
pub struct CompatibleEnchantmentsEnchant {
    pub enchanter: Rc<dyn Enchant>,
    pub enchantment_compatibility: Rc<dyn EnchantmentsCompatible>,
}

impl CompatibleEnchantmentsEnchant {
    pub fn wrap(
        enchanter: Rc<dyn Enchant>,
        enchantment_compatibility: Rc<dyn EnchantmentsCompatible>,
    ) -> Self {
        Self {
            enchanter,
            enchantment_compatibility,
        }
    }

    pub fn new(enchantment_compatibility: Rc<dyn EnchantmentsCompatible>) -> Self {
        Self::wrap(Rc::new(AgnosticEnchant), enchantment_compatibility)
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

impl Enchant for CompatibleEnchantmentsEnchant {
    fn enchant(&self, item: &mut Item, enchantment: Enchantment) -> Result<()> {
        match self.new_enchantment_compatible(item.enchantment_kinds(), enchantment.kind) {
            true => self.enchanter.enchant(item, enchantment),
            false => Err(Error::EnchantmentsIncompatible),
        }
    }
}

pub trait RequireCompatibleEnchantments {
    fn require_compatible_enchantments(
        self,
        enchantment_compatibility: Rc<dyn EnchantmentsCompatible>,
    ) -> CompatibleEnchantmentsEnchant
    where
        Self: Sized;
}

impl RequireCompatibleEnchantments for Rc<dyn Enchant> {
    fn require_compatible_enchantments(
        self,
        enchantment_compatibility: Rc<dyn EnchantmentsCompatible>,
    ) -> CompatibleEnchantmentsEnchant
    where
        Self: Sized,
    {
        CompatibleEnchantmentsEnchant::wrap(self, enchantment_compatibility)
    }
}
