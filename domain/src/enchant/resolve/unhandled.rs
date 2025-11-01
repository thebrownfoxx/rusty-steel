use crate::enchant::resolve::{EnchantmentsResolution, ResolveEnchantments};
use crate::enchantment::Enchantment;

#[derive(Debug)]
pub struct UnhandledEnchantmentsResolver;

impl ResolveEnchantments for UnhandledEnchantmentsResolver {
    fn resolve(&self, _: &mut Enchantment, _: &Enchantment) -> EnchantmentsResolution {
        EnchantmentsResolution::Unresolved
    }
}
