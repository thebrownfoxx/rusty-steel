use super::{AgnosticEnchant, Result};
use super::{Enchant, Error};
use crate::enchantment::{Enchantment, EnchantmentKindId, EnchantmentsCompatible};
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
        enchantment_a: &dyn AsRef<EnchantmentKindId>,
        enchantment_b: &dyn AsRef<EnchantmentKindId>,
    ) -> bool {
        self.enchantment_compatibility
            .are_compatible(enchantment_a, enchantment_b)
    }

    fn new_enchantment_compatible<'a>(
        &self,
        mut existing_enchantments: impl Iterator<Item = &'a EnchantmentKindId>,
        new_enchantment: impl AsRef<EnchantmentKindId>,
    ) -> bool {
        existing_enchantments.all(|existing| self.are_compatible(&existing, &new_enchantment))
    }
}

impl Enchant for CompatibleEnchantmentsEnchant {
    fn enchant(&self, item: &mut Item, enchantment: Enchantment) -> Result<()> {
        match self.new_enchantment_compatible(item.enchantment_kinds(), &enchantment.kind) {
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
