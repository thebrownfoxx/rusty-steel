use crate::enchant::resolve::{EnchantmentsResolution, ResolveEnchantments};
use crate::enchant::{Enchant, EnchantError, EnchantResult};
use crate::enchantment::Enchantment;
use crate::item::Item;

pub struct ResolveEnchantmentsEnchanter<Resolve: ResolveEnchantments, Fallback: Enchant> {
    resolver: Resolve,
    fallback: Fallback,
}

impl<Resolve: ResolveEnchantments, Fallback: Enchant> Enchant
    for ResolveEnchantmentsEnchanter<Resolve, Fallback>
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

pub trait ResolveEnchantmentsEnchant<Resolve: ResolveEnchantments, Fallback: Enchant> {
    fn resolve_enchantments(
        self,
        resolver: Resolve,
    ) -> ResolveEnchantmentsEnchanter<Resolve, Fallback>;
}

impl<Resolve: ResolveEnchantments, Fallback: Enchant> ResolveEnchantmentsEnchant<Resolve, Fallback>
    for Fallback
{
    fn resolve_enchantments(
        self,
        resolver: Resolve,
    ) -> ResolveEnchantmentsEnchanter<Resolve, Fallback> {
        ResolveEnchantmentsEnchanter {
            resolver,
            fallback: self,
        }
    }
}
