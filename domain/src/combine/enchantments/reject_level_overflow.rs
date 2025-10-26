use super::{Combine, Error, Result};
use crate::enchantment::{Enchantment, MaxLevel};

pub struct RejectLevelOverflowCombine<Impl: Combine, Max: MaxLevel> {
    implementation: Impl,
    max_level_provider: Max,
}

impl<Impl: Combine, Cap: MaxLevel> Combine for RejectLevelOverflowCombine<Impl, Cap> {
    fn combine(&self, target: &mut Enchantment, sacrifice: &Enchantment) -> Result {
        let original_level = target.level;

        let () = self.implementation.combine(target, sacrifice)?;

        let Some(max_level) = self.max_level_provider.max_level(&target) else {
            return Ok(());
        };

        if target.level <= max_level {
            return Ok(());
        }

        target.level = original_level;
        Err(Error::LevelsIncompatible)
    }
}

pub trait RejectLevelOverflow<Impl: Combine, Max: MaxLevel> {
    fn reject_level_overflow(
        self,
        max_level_provider: Max,
    ) -> RejectLevelOverflowCombine<Impl, Max>;
}

impl<Impl: Combine, Max: MaxLevel> RejectLevelOverflow<Impl, Max> for Impl {
    fn reject_level_overflow(
        self,
        max_level_provider: Max,
    ) -> RejectLevelOverflowCombine<Impl, Max> {
        RejectLevelOverflowCombine {
            implementation: self,
            max_level_provider,
        }
    }
}
