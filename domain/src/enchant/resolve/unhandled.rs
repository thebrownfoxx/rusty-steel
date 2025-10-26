use crate::enchant::resolve::{EnchantmentsResolution, ResolveEnchantments};
use crate::enchantment::Enchantment;

#[derive(Debug)]
pub struct UnhandledEnchantmentResolver;

impl ResolveEnchantments for UnhandledEnchantmentResolver {
    fn resolve(&self, _: &mut Enchantment, _: &Enchantment) -> EnchantmentsResolution {
        EnchantmentsResolution::Unhandled
    }
}
