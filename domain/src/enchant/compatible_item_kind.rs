use crate::compatible::AreCompatible;
use crate::enchant::{Enchant, EnchantError, EnchantResult};
use crate::enchantment::{Enchantment, EnchantmentKindId};
use crate::item::{Item, ItemKindId};

pub struct CompatibleItemKindEnchant<Impl, Compat>
where
    Impl: Enchant,
    Compat: AreCompatible<ItemKindId, EnchantmentKindId>,
{
    implementation: Impl,
    compatibility: Compat,
}

impl<Impl, Compat> Enchant for CompatibleItemKindEnchant<Impl, Compat>
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

pub trait RequireCompatibleItemKind<Impl, Compat>
where
    Impl: Enchant,
    Compat: AreCompatible<ItemKindId, EnchantmentKindId>,
{
    fn require_compatible_item_kind(
        self,
        item_enchantment_compatibility: Compat,
    ) -> CompatibleItemKindEnchant<Impl, Compat>;
}

impl<Impl, Compat> RequireCompatibleItemKind<Impl, Compat> for Impl
where
    Impl: Enchant,
    Compat: AreCompatible<ItemKindId, EnchantmentKindId>,
{
    fn require_compatible_item_kind(
        self,
        compatibility: Compat,
    ) -> CompatibleItemKindEnchant<Impl, Compat> {
        CompatibleItemKindEnchant {
            implementation: self,
            compatibility,
        }
    }
}
