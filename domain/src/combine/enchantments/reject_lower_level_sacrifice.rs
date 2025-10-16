use super::{Combine, Error, Result};
use crate::enchantment::Enchantment;

pub struct RejectLowerLevelSacrifice<Impl: Combine>(pub Impl);

impl<Impl: Combine> Combine for RejectLowerLevelSacrifice<Impl> {
    fn combine(&self, target: &mut Enchantment, sacrifice: &Enchantment) -> Result {
        if sacrifice.level < target.level {
            return Err(Error::LevelsIncompatible);
        }

        Ok(())
    }
}
