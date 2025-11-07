use super::{CombineEnchantmentLevels, CombineEnchantmentsError, CombineEnchantmentsResult};
use crate::builder::Builder;
use crate::enchantment::Enchantment;

#[derive(Debug)]
pub struct RejectLowerLevelSacrifice<Impl: CombineEnchantmentLevels>(Impl);

impl<Impl: CombineEnchantmentLevels> CombineEnchantmentLevels for RejectLowerLevelSacrifice<Impl> {
    fn combine(
        &self,
        target: &mut Enchantment,
        sacrifice: &Enchantment,
    ) -> CombineEnchantmentsResult {
        if sacrifice.level < target.level {
            return Err(CombineEnchantmentsError::IncompatibleLevels);
        }

        Ok(())
    }
}

pub trait RejectLowerLevelSacrificeBuilder<Impl: CombineEnchantmentLevels> {
    fn reject_lower_level_sacrifice(self) -> Builder<RejectLowerLevelSacrifice<Impl>>;
}

impl<Impl: CombineEnchantmentLevels> RejectLowerLevelSacrificeBuilder<Impl> for Builder<Impl> {
    fn reject_lower_level_sacrifice(self) -> Builder<RejectLowerLevelSacrifice<Impl>> {
        self.reimplement(|implementation| RejectLowerLevelSacrifice(implementation))
    }
}
