use super::{Enchant, Error, Result};
use crate::compatible::AreCompatible;
use crate::enchantment::{Enchantment, EnchantmentKindId};
use crate::item::Item;
use std::rc::Rc;

pub struct CompatibleEnchantments<Impl, Compat>
where
    Impl: Enchant,
    Compat: AreCompatible<EnchantmentKindId>,
{
    implementation: Impl,
    compatibility: Rc<Compat>,
}

impl<Impl, Compat> CompatibleEnchantments<Impl, Compat>
where
    Impl: Enchant,
    Compat: AreCompatible<EnchantmentKindId>
{
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

impl<Impl, Compat> Enchant
    for CompatibleEnchantments<Impl, Compat>
where
    Impl: Enchant,
    Compat: AreCompatible<EnchantmentKindId>
{
    fn enchant(&self, item: &mut Item, enchantment: Enchantment) -> Result<()> {
        if let Some(conflict) = self.get_conflict(item.enchantment_kinds(), &enchantment) {
            return Err(Error::EnchantmentsIncompatible { conflict });
        }

        self.implementation.enchant(item, enchantment)
    }
}

pub trait RequireCompatibleEnchantments<Impl, Compat>
where
    Impl: Enchant,
    Compat: AreCompatible<EnchantmentKindId>
{
    fn require_compatible_enchantments(
        self,
        enchantment_compatible: Rc<Compat>,
    ) -> CompatibleEnchantments<Impl, Compat>;
}

impl<Impl, Compat> RequireCompatibleEnchantments<Impl, Compat> for Impl
where
    Impl: Enchant,
    Compat: AreCompatible<EnchantmentKindId>
{
    fn require_compatible_enchantments(
        self,
        compatibility: Rc<Compat>,
    ) -> CompatibleEnchantments<Impl, Compat> {
        CompatibleEnchantments {
            implementation: self,
            compatibility,
        }
    }
}
