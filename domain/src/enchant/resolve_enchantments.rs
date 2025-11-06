use crate::builder::Builder;
use crate::enchant::resolve::{EnchantmentsResolution, ResolveEnchantments};
use crate::enchant::{Enchant, EnchantError, EnchantResult};
use crate::enchantment::Enchantment;
use crate::item::Item;

#[derive(Debug)]
pub struct IterateEnchantments<Resolve: ResolveEnchantments, Fallback: Enchant> {
    resolver: Resolve,
    fallback: Fallback,
}

impl<Resolve: ResolveEnchantments, Fallback: Enchant> Enchant
    for IterateEnchantments<Resolve, Fallback>
{
    fn enchant(&self, item: &mut Item, enchantment: Enchantment) -> EnchantResult {
        for target in item.enchantment_levels.iter_mut() {
            let resolution = self.resolver.resolve(target, &enchantment);

            match resolution {
                EnchantmentsResolution::Resolved => return Ok(()),
                EnchantmentsResolution::Unresolved => continue,
                EnchantmentsResolution::Incompatible => {
                    let conflict = target.kind.clone();
                    return Err(EnchantError::IncompatibleEnchantments { conflict });
                }
            }
        }

        self.fallback.enchant(item, enchantment)
    }
}

pub trait IterateEnchantmentsBuilder<Resolve: ResolveEnchantments, Fallback: Enchant> {
    fn resolve_enchantments(
        self,
        resolver: Resolve,
    ) -> Builder<IterateEnchantments<Resolve, Fallback>>;
}

impl<Resolve: ResolveEnchantments, Fallback: Enchant> IterateEnchantmentsBuilder<Resolve, Fallback>
    for Builder<Fallback>
{
    fn resolve_enchantments(
        self,
        resolver: Resolve,
    ) -> Builder<IterateEnchantments<Resolve, Fallback>> {
        self.reimplement(|fallback| IterateEnchantments { resolver, fallback })
    }
}
