use crate::builder::Builder;
use crate::compatible::AreCompatible;
use crate::enchant::{Enchant, EnchantError, EnchantResult};
use crate::enchantment::{Enchantment, EnchantmentKindId};
use crate::item::{Item, ItemKindId};

#[derive(Debug)]
pub struct RejectIncompatibleItem<Impl, Compat>
where
    Impl: Enchant,
    Compat: AreCompatible<ItemKindId, EnchantmentKindId>,
{
    implementation: Impl,
    compatibility: Compat,
}

impl<Impl, Compat> Enchant for RejectIncompatibleItem<Impl, Compat>
where
    Impl: Enchant,
    Compat: AreCompatible<ItemKindId, EnchantmentKindId>,
{
    fn enchant(&self, item: &mut Item, enchantment: Enchantment) -> EnchantResult {
        let compatibility = &self.compatibility;

        if !compatibility.are_compatible(&item, &enchantment) {
            return Err(EnchantError::ItemKindIncompatible);
        }

        self.implementation.enchant(item, enchantment)
    }
}

pub trait RejectIncompatibleItemBuilder<Impl, Compat>
where
    Impl: Enchant,
    Compat: AreCompatible<ItemKindId, EnchantmentKindId>,
{
    fn reject_incompatible_item(
        self,
        item_enchantment_compatibility: Compat,
    ) -> Builder<RejectIncompatibleItem<Impl, Compat>>;
}

impl<Impl, Compat> RejectIncompatibleItemBuilder<Impl, Compat> for Builder<Impl>
where
    Impl: Enchant,
    Compat: AreCompatible<ItemKindId, EnchantmentKindId>,
{
    fn reject_incompatible_item(
        self,
        compatibility: Compat,
    ) -> Builder<RejectIncompatibleItem<Impl, Compat>> {
        self.reimplement(|implementation| RejectIncompatibleItem {
            implementation,
            compatibility,
        })
    }
}
