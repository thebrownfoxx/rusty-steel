use crate::enchant::resolve::{EnchantmentsResolution, ResolveEnchantments};
use crate::enchant::{Enchant, EnchantError, EnchantResult};
use crate::enchantment::Enchantment;
use crate::item::Item;

pub struct ResolvingEnchant<Resolve: ResolveEnchantments, Fallback: Enchant>
{
    resolver: Resolve,
    fallback: Fallback,
}

impl<Resolve: ResolveEnchantments, Fallback: Enchant> Enchant
    for ResolvingEnchant<Resolve, Fallback>
{
    fn enchant(&self, item: &mut Item, enchantment: Enchantment) -> EnchantResult {
        for target in item.enchantments.iter_mut() {
            let resolution = self.resolver.resolve(target, &enchantment);

            match resolution {
                EnchantmentsResolution::Handled => return Ok(()),
                EnchantmentsResolution::Unhandled => continue,
                EnchantmentsResolution::Incompatible { conflict } => {
                    return Err(EnchantError::EnchantmentsIncompatible { conflict });
                }
            }
        }

        self.fallback.enchant(item, enchantment)
    }
}

pub trait ResolveAgainstTargetEnchantments<Resolve: ResolveEnchantments, Fallback: Enchant> {
    fn resolve_against_target_enchantments(
        self,
        resolver: Resolve,
    ) -> ResolvingEnchant<Resolve, Fallback>;
}

impl<Resolve: ResolveEnchantments, Fallback: Enchant>
    ResolveAgainstTargetEnchantments<Resolve, Fallback> for Fallback
{
    fn resolve_against_target_enchantments(
        self,
        resolver: Resolve,
    ) -> ResolvingEnchant<Resolve, Fallback> {
        ResolvingEnchant {
            resolver,
            fallback: self,
        }
    }
}
