use crate::compatible::AreCompatible;
use crate::enchant::resolve::{EnchantmentsResolution, ResolveEnchantments};
use crate::enchantment::{Enchantment, EnchantmentKindId};

#[derive(Debug)]
pub struct RequireCompatibleResolveEnchantments<Impl, Compat>
where
    Impl: ResolveEnchantments,
    Compat: AreCompatible<EnchantmentKindId>,
{
    implementation: Impl,
    compatibility: Compat,
}

impl<Impl, Compat> ResolveEnchantments for RequireCompatibleResolveEnchantments<Impl, Compat>
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

pub trait RequireCompatible<Impl, Compat>
where
    Impl: ResolveEnchantments,
    Compat: AreCompatible<EnchantmentKindId>,
{
    fn require_compatible(
        self,
        compatibility: Compat,
    ) -> RequireCompatibleResolveEnchantments<Impl, Compat>;
}

impl<Impl, Compat> RequireCompatible<Impl, Compat> for Impl
where
    Impl: ResolveEnchantments,
    Compat: AreCompatible<EnchantmentKindId>,
{
    fn require_compatible(
        self,
        compatibility: Compat,
    ) -> RequireCompatibleResolveEnchantments<Impl, Compat> {
        RequireCompatibleResolveEnchantments {
            implementation: self,
            compatibility,
        }
    }
}
