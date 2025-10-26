use super::{Combine, Error, Result};
use crate::enchantment::Enchantment;

pub struct RejectLowerLevelSacrificeCombine<Impl: Combine>(Impl);

impl<Impl: Combine> Combine for RejectLowerLevelSacrificeCombine<Impl> {
    fn combine(&self, target: &mut Enchantment, sacrifice: &Enchantment) -> Result {
        if sacrifice.level < target.level {
            return Err(Error::LevelsIncompatible);
        }

        Ok(())
    }
}

pub trait RejectLowerLevelSacrifice<Impl: Combine> {
    fn reject_lower_level_sacrifice(self) -> RejectLowerLevelSacrificeCombine<Impl>;
}

impl<Impl: Combine> RejectLowerLevelSacrifice<Impl> for Impl {
    fn reject_lower_level_sacrifice(self) -> RejectLowerLevelSacrificeCombine<Impl> {
        RejectLowerLevelSacrificeCombine(self)
    }
}
