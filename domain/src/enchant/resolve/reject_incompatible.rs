use crate::builder::Builder;
use crate::compatible::AreCompatible;
use crate::enchant::resolve::{EnchantmentsResolution, ResolveEnchantments};
use crate::enchantment::{Enchantment, EnchantmentKindId};

#[derive(Debug)]
pub struct RejectIncompatible<Impl, Compat>
where
    Impl: ResolveEnchantments,
    Compat: AreCompatible<EnchantmentKindId>,
{
    implementation: Impl,
    compatibility: Compat,
}

impl<Impl, Compat> ResolveEnchantments for RejectIncompatible<Impl, Compat>
where
    Impl: ResolveEnchantments,
    Compat: AreCompatible<EnchantmentKindId>,
{
    fn resolve(&self, target: &mut Enchantment, sacrifice: &Enchantment) -> EnchantmentsResolution {
        if !self.compatibility.are_compatible(target, sacrifice) {
            return EnchantmentsResolution::Incompatible;
        }

        self.implementation.resolve(target, sacrifice)
    }
}

pub trait RejectIncompatibleBuilder<Impl, Compat>
where
    Impl: ResolveEnchantments,
    Compat: AreCompatible<EnchantmentKindId>,
{
    fn reject_incompatible(
        self,
        compatibility: Compat,
    ) -> Builder<RejectIncompatible<Impl, Compat>>;
}

impl<Impl, Compat> RejectIncompatibleBuilder<Impl, Compat> for Builder<Impl>
where
    Impl: ResolveEnchantments,
    Compat: AreCompatible<EnchantmentKindId>,
{
    fn reject_incompatible(
        self,
        compatibility: Compat,
    ) -> Builder<RejectIncompatible<Impl, Compat>> {
        self.reimplement(|implementation| RejectIncompatible {
            implementation,
            compatibility,
        })
    }
}
