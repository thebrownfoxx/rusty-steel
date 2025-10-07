use super::{Enchant, Error, ErrorKind, Result};
use crate::enchantment::{AreCompatible, Enchantment, EnchantmentKindId};
use crate::item::Item;
use std::rc::Rc;

pub struct CompatibleEnchantments<Impl, Compat>
where
    Impl: Enchant,
    Compat: AreCompatible,
{
    enchant: Impl,
    compatibility: Rc<Compat>,
}

impl<Impl, Compat> CompatibleEnchantments<Impl, Compat>
where
    Impl: Enchant,
    Compat: AreCompatible,
{
    fn are_compatible(
        &self,
        enchantment_a: &impl AsRef<EnchantmentKindId>,
        enchantment_b: &impl AsRef<EnchantmentKindId>,
    ) -> bool {
        self.compatibility
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

impl<Impl, Compat> Enchant for CompatibleEnchantments<Impl, Compat>
where
    Impl: Enchant,
    Compat: AreCompatible,
{
    fn enchant(&self, item: &mut Item, enchantment: Enchantment) -> Result<()> {
        if !self.new_enchantment_compatible(item.enchantment_kinds(), &enchantment.kind) {
            return Err(Error {
                enchantment,
                kind: ErrorKind::EnchantmentsIncompatible,
            });
        }

        self.enchant.enchant(item, enchantment)
    }
}

pub trait RequireCompatibleEnchantments<Impl, Compat>
where
    Impl: Enchant,
    Compat: AreCompatible,
{
    fn require_compatible_enchantments(
        self,
        enchantment_compatible: Rc<Compat>,
    ) -> CompatibleEnchantments<Impl, Compat>;
}

impl<Impl, Compat> RequireCompatibleEnchantments<Impl, Compat> for Impl
where
    Impl: Enchant,
    Compat: AreCompatible,
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
