use super::{CombineEnchantments, CombineEnchantmentsError, CombineEnchantmentsResult};
use crate::enchantment::Enchantment;

#[derive(Debug)]
pub struct RejectLowerLevelSacrificeEnchantmentCombiner<Impl: CombineEnchantments>(Impl);

impl<Impl: CombineEnchantments> CombineEnchantments
    for RejectLowerLevelSacrificeEnchantmentCombiner<Impl>
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

pub trait RejectLowerLevelSacrificeCombineEnchantments<Impl: CombineEnchantments> {
    fn reject_lower_level_sacrifice(self) -> RejectLowerLevelSacrificeEnchantmentCombiner<Impl>;
}

impl<Impl: CombineEnchantments> RejectLowerLevelSacrificeCombineEnchantments<Impl> for Impl {
    fn reject_lower_level_sacrifice(self) -> RejectLowerLevelSacrificeEnchantmentCombiner<Impl> {
        RejectLowerLevelSacrificeEnchantmentCombiner(self)
    }
}
