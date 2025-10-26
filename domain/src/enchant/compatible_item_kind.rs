use crate::compatible::AreCompatible;
use crate::enchant::{Enchant, Error, Result};
use crate::enchantment::{Enchantment, EnchantmentKindId};
use crate::item::{Item, ItemKindId};
use std::rc::Rc;

pub struct CompatibleItemKind<Impl, Compat>
where
    Impl: Enchant,
    Compat: AreCompatible<ItemKindId, EnchantmentKindId>,
{
    implementation: Impl,
    compatibility: Rc<Compat>,
}

impl<Impl, Compat> Enchant for CompatibleItemKind<Impl, Compat>
where
    Impl: Enchant,
    Compat: AreCompatible<ItemKindId, EnchantmentKindId>,
{
    fn enchant(&self, item: &mut Item, enchantment: Enchantment) -> Result<()> {
        let compatibility = &self.compatibility;

        if !compatibility.are_compatible(&item, &enchantment) {
            return Err(Error::ItemKindIncompatible);
        }

        self.implementation.enchant(item, enchantment)
    }
}

pub trait RequireCompatibleItemKind<Impl, Compat>
where
    Impl: Enchant,
    Compat: AreCompatible<ItemKindId, EnchantmentKindId>,
{
    fn require_compatible_item_kind(
        self,
        item_enchantment_compatibility: Rc<Compat>,
    ) -> CompatibleItemKind<Impl, Compat>;
}

impl<Impl, Compat> RequireCompatibleItemKind<Impl, Compat> for Impl
where
    Impl: Enchant,
    Compat: AreCompatible<ItemKindId, EnchantmentKindId>,
{
    fn require_compatible_item_kind(
        self,
        compatibility: Rc<Compat>,
    ) -> CompatibleItemKind<Impl, Compat> {
        CompatibleItemKind {
            implementation: self,
            compatibility,
        }
    }
}
