use crate::enchant::resolve::{EnchantmentsResolution, ResolveEnchantments};
use crate::enchantment::Enchantment;

pub struct UnhandledResolveEnchantments;

impl ResolveEnchantments for UnhandledResolveEnchantments {
    fn resolve(&self, _: &mut Enchantment, _: &Enchantment) -> EnchantmentsResolution {
        EnchantmentsResolution::Unhandled
    }
}
