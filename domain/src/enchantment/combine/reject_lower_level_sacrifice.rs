use super::{CombineEnchantments, CombineEnchantmentsError, CombineEnchantmentsResult};
use crate::builder::Builder;
use crate::enchantment::Enchantment;

#[derive(Debug)]
pub struct RejectLowerLevelSacrifice<Impl: CombineEnchantments>(Impl);

impl<Impl: CombineEnchantments> CombineEnchantments for RejectLowerLevelSacrifice<Impl> {
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

pub trait RejectLowerLevelSacrificeBuilder<Impl: CombineEnchantments> {
    fn reject_lower_level_sacrifice(self) -> Builder<RejectLowerLevelSacrifice<Impl>>;
}

impl<Impl: CombineEnchantments> RejectLowerLevelSacrificeBuilder<Impl> for Builder<Impl> {
    fn reject_lower_level_sacrifice(self) -> Builder<RejectLowerLevelSacrifice<Impl>> {
        self.reimplement(|implementation| RejectLowerLevelSacrifice(implementation))
    }
}
