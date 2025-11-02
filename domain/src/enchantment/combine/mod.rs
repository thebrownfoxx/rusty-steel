mod max_level_capped;
mod reject_level_overflow;
mod reject_lower_level_sacrifice;
mod standard;

pub use max_level_capped::{MaxLevelCapped, MaxLevelCappedBuilder};
pub use reject_level_overflow::{RejectLevelOverflow, RejectLevelOverflowBuilder};
pub use reject_lower_level_sacrifice::{
    RejectLowerLevelSacrifice, RejectLowerLevelSacrificeBuilder,
};
pub use standard::StandardEnchantmentCombiner;

use crate::enchantment::Enchantment;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

pub trait CombineEnchantments {
    fn combine(
        &self,
        target: &mut Enchantment,
        sacrifice: &Enchantment,
    ) -> CombineEnchantmentsResult;
}

impl<Wrapper: Deref<Target: CombineEnchantments>> CombineEnchantments for Wrapper {
    fn combine(
        &self,
        target: &mut Enchantment,
        sacrifice: &Enchantment,
    ) -> CombineEnchantmentsResult {
        self.deref().combine(target, sacrifice)
    }
}

pub type CombineEnchantmentsResult<T = ()> = Result<T, CombineEnchantmentsError>;

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub enum CombineEnchantmentsError {
    IncompatibleKinds,
    IncompatibleLevels,
}

impl Display for CombineEnchantmentsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for CombineEnchantmentsError {}
