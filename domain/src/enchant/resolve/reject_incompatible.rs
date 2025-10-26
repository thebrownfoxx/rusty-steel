use crate::compatible::AreCompatible;
use crate::enchant::resolve::{EnchantmentsResolution, ResolveEnchantments};
use crate::enchantment::{Enchantment, EnchantmentKindId};

#[derive(Debug)]
pub struct RejectIncompatibleEnchantmentResolver<Impl, Compat>
where
    Impl: ResolveEnchantments,
    Compat: AreCompatible<EnchantmentKindId>,
{
    implementation: Impl,
    compatibility: Compat,
}

impl<Impl, Compat> ResolveEnchantments for RejectIncompatibleEnchantmentResolver<Impl, Compat>
where
    Impl: ResolveEnchantments,
    Compat: AreCompatible<EnchantmentKindId>,
{
    fn resolve(&self, target: &mut Enchantment, sacrifice: &Enchantment) -> EnchantmentsResolution {
        if !self.compatibility.are_compatible(target, sacrifice) {
            return EnchantmentsResolution::Incompatible {
                conflict: target.kind.clone(),
            };
        }

        self.implementation.resolve(target, sacrifice)
    }
}

pub trait RejectIncompatibleResolveEnchantments<Impl, Compat>
where
    Impl: ResolveEnchantments,
    Compat: AreCompatible<EnchantmentKindId>,
{
    fn reject_incompatible(
        self,
        compatibility: Compat,
    ) -> RejectIncompatibleEnchantmentResolver<Impl, Compat>;
}

impl<Impl, Compat> RejectIncompatibleResolveEnchantments<Impl, Compat> for Impl
where
    Impl: ResolveEnchantments,
    Compat: AreCompatible<EnchantmentKindId>,
{
    fn reject_incompatible(
        self,
        compatibility: Compat,
    ) -> RejectIncompatibleEnchantmentResolver<Impl, Compat> {
        RejectIncompatibleEnchantmentResolver {
            implementation: self,
            compatibility,
        }
    }
}
