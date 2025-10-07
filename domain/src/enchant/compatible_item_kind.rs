use crate::enchant::{Enchant, Error, ErrorKind, Result};
use crate::enchantment::{Enchantment, EnchantmentKindId};
use crate::item::{AreCompatible, Item, ItemKindId};
use std::rc::Rc;

pub struct CompatibleItemKind<Impl, Compat>
where
    Impl: Enchant,
    Compat: AreCompatible,
{
    pub enchant: Impl,
    pub item_enchantment_compatible: Rc<Compat>,
}

impl<Impl, Compat> CompatibleItemKind<Impl, Compat>
where
    Impl: Enchant,
    Compat: AreCompatible,
{
    fn are_compatible(
        &self,
        item: &impl AsRef<ItemKindId>,
        enchantment: &impl AsRef<EnchantmentKindId>,
    ) -> bool {
        self.item_enchantment_compatible
            .are_compatible(item, enchantment)
    }
}

impl<Impl, Compat> Enchant for CompatibleItemKind<Impl, Compat>
where
    Impl: Enchant,
    Compat: AreCompatible,
{
    fn enchant(&self, item: &mut Item, enchantment: Enchantment) -> Result<()> {
        if !self.are_compatible(&item.kind, &enchantment.kind) {
            return Err(Error {
                enchantment,
                kind: ErrorKind::EnchantmentsIncompatible,
            });
        }

        self.enchant.enchant(item, enchantment)
    }
}

pub trait RequireCompatibleItemKind<Impl, Compat>
where
    Impl: Enchant,
    Compat: AreCompatible,
{
    fn require_compatible_item_kind(
        self,
        item_enchantment_compatibility: Rc<Compat>,
    ) -> CompatibleItemKind<Impl, Compat>;
}

impl<Impl, Compat> RequireCompatibleItemKind<Impl, Compat> for Impl
where
    Impl: Enchant,
    Compat: AreCompatible,
{
    fn require_compatible_item_kind(
        self,
        item_enchantment_compatible: Rc<Compat>,
    ) -> CompatibleItemKind<Impl, Compat> {
        CompatibleItemKind {
            enchant: self,
            item_enchantment_compatible,
        }
    }
}
