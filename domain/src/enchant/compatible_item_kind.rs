use crate::enchant::{Enchant, Error, Result};
use crate::enchantment::{Enchantment, EnchantmentKindId};
use crate::item;
use crate::item::{Item, ItemKindId};
use std::rc::Rc;

pub struct CompatibleItemKind<Impl: Enchant, Compat: item::EnchantmentCompatible> {
    enchant: Impl,
    compatibility: Rc<Compat>,
}

impl<Impl: Enchant, Compat: item::EnchantmentCompatible> Enchant
    for CompatibleItemKind<Impl, Compat>
{
    fn enchant(&self, item: &mut Item, enchantment: Enchantment) -> Result<()> {
        let compatibility = &self.compatibility;

        if !compatibility.are_compatible(&item, &enchantment) {
            return Err(Error::ItemKindIncompatible);
        }

        self.enchant.enchant(item, enchantment)
    }
}

pub trait RequireCompatibleItemKind<Impl, Compat>
where
    Impl: Enchant,
    Compat: item::EnchantmentCompatible,
{
    fn require_compatible_item_kind(
        self,
        item_enchantment_compatibility: Rc<Compat>,
    ) -> CompatibleItemKind<Impl, Compat>;
}

impl<Impl: Enchant, Compat: item::EnchantmentCompatible> RequireCompatibleItemKind<Impl, Compat>
    for Impl
{
    fn require_compatible_item_kind(
        self,
        compatibility: Rc<Compat>,
    ) -> CompatibleItemKind<Impl, Compat> {
        CompatibleItemKind {
            enchant: self,
            compatibility,
        }
    }
}
