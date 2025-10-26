use crate::compatible::AreCompatible;
use crate::enchant::{Enchant, EnchantError, EnchantResult};
use crate::enchantment::{Enchantment, EnchantmentKindId};
use crate::item::{Item, ItemKindId};

#[derive(Debug)]
pub struct RequireCompatibleItemEnchanter<Impl, Compat>
where
    Impl: Enchant,
    Compat: AreCompatible<ItemKindId, EnchantmentKindId>,
{
    implementation: Impl,
    compatibility: Compat,
}

impl<Impl, Compat> Enchant for RequireCompatibleItemEnchanter<Impl, Compat>
where
    Impl: Enchant,
    Compat: AreCompatible<ItemKindId, EnchantmentKindId>,
{
    fn enchant(&self, item: &mut Item, enchantment: Enchantment) -> EnchantResult<()> {
        let compatibility = &self.compatibility;

        if !compatibility.are_compatible(&item, &enchantment) {
            return Err(EnchantError::ItemKindIncompatible);
        }

        self.implementation.enchant(item, enchantment)
    }
}

pub trait RequireCompatibleItemEnchant<Impl, Compat>
where
    Impl: Enchant,
    Compat: AreCompatible<ItemKindId, EnchantmentKindId>,
{
    fn require_compatible_item(
        self,
        item_enchantment_compatibility: Compat,
    ) -> RequireCompatibleItemEnchanter<Impl, Compat>;
}

impl<Impl, Compat> RequireCompatibleItemEnchant<Impl, Compat> for Impl
where
    Impl: Enchant,
    Compat: AreCompatible<ItemKindId, EnchantmentKindId>,
{
    fn require_compatible_item(
        self,
        compatibility: Compat,
    ) -> RequireCompatibleItemEnchanter<Impl, Compat> {
        RequireCompatibleItemEnchanter {
            implementation: self,
            compatibility,
        }
    }
}
