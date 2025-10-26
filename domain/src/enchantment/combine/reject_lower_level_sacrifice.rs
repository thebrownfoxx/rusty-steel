use super::{CombineEnchantments, CombineEnchantmentsError, CombineEnchantmentsResult};
use crate::enchantment::Enchantment;

pub struct RejectLowerLevelSacrificeCombineEnchantments<Impl: CombineEnchantments>(Impl);

impl<Impl: CombineEnchantments> CombineEnchantments
    for RejectLowerLevelSacrificeCombineEnchantments<Impl>
{
    fn combine(
        &self,
        target: &mut Enchantment,
        sacrifice: &Enchantment,
    ) -> CombineEnchantmentsResult {
        if sacrifice.level < target.level {
            return Err(CombineEnchantmentsError::LevelsIncompatible);
        }

        Ok(())
    }
}

pub trait RejectLowerLevelSacrifice<Impl: CombineEnchantments> {
    fn reject_lower_level_sacrifice(self) -> RejectLowerLevelSacrificeCombineEnchantments<Impl>;
}

impl<Impl: CombineEnchantments> RejectLowerLevelSacrifice<Impl> for Impl {
    fn reject_lower_level_sacrifice(self) -> RejectLowerLevelSacrificeCombineEnchantments<Impl> {
        RejectLowerLevelSacrificeCombineEnchantments(self)
    }
}
