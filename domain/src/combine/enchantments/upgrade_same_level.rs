use super::{Combine, Error, ErrorKind, Result};
use crate::enchantment::Enchantment;

pub struct UpgradeSameLevel;

impl Combine for UpgradeSameLevel {
    fn combine(&self, target: &mut Enchantment, sacrifice: Enchantment) -> Result {
        if target.level != sacrifice.level {
            return Err(Error {
                sacrifice,
                kind: ErrorKind::IncompatibleLevels,
            });
        }

        target.level += 1;
        Ok(())
    }
}
