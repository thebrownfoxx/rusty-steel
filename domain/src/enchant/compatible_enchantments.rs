use super::{Enchant, Error, Result};
use crate::enchantment;
use crate::enchantment::{Enchantment, EnchantmentKindId};
use crate::item::Item;
use std::rc::Rc;

pub struct CompatibleEnchantments<Impl: Enchant, Compat: enchantment::Compatible> {
    enchant: Impl,
    compatibility: Rc<Compat>,
}

impl<Impl: Enchant, Compat: enchantment::Compatible> CompatibleEnchantments<Impl, Compat> {
    fn get_conflict<'a>(
        &self,
        mut existing: impl Iterator<Item = &'a EnchantmentKindId>,
        new: impl AsRef<EnchantmentKindId>,
    ) -> Option<EnchantmentKindId> {
        let compatibility = &self.compatibility;

        existing
            .find(|existing| !compatibility.are_compatible(&existing, &new))
            .cloned()
    }
}

impl<Impl: Enchant, Compat: enchantment::Compatible> Enchant
    for CompatibleEnchantments<Impl, Compat>
{
    fn enchant(&self, item: &mut Item, enchantment: Enchantment) -> Result<()> {
        if let Some(conflict) = self.get_conflict(item.enchantment_kinds(), &enchantment) {
            return Err(Error::EnchantmentsIncompatible { conflict });
        }

        self.enchant.enchant(item, enchantment)
    }
}

pub trait RequireCompatibleEnchantments<Impl: Enchant, Compat: enchantment::Compatible> {
    fn require_compatible_enchantments(
        self,
        enchantment_compatible: Rc<Compat>,
    ) -> CompatibleEnchantments<Impl, Compat>;
}

impl<Impl: Enchant, Compat: enchantment::Compatible> RequireCompatibleEnchantments<Impl, Compat>
    for Impl
{
    fn require_compatible_enchantments(
        self,
        compatibility: Rc<Compat>,
    ) -> CompatibleEnchantments<Impl, Compat> {
        CompatibleEnchantments {
            enchant: self,
            compatibility,
        }
    }
}
