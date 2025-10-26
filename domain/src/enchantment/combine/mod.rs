mod max_level_capped;
mod reject_level_overflow;
mod reject_lower_level_sacrifice;
mod standard;

pub use max_level_capped::{MaxLevelCappedCombineEnchantments, MaxLevelCappedEnchantmentCombiner};
pub use reject_level_overflow::{
    RejectLevelOverflowCombineEnchantments, RejectLevelOverflowEnchantmentCombiner,
};
pub use reject_lower_level_sacrifice::{
    RejectLowerLevelSacrificeCombineEnchantments, RejectLowerLevelSacrificeEnchantmentCombiner,
};
pub use standard::StandardEnchantmentCombiner;
use std::fmt::{Display, Formatter};

use crate::enchantment::Enchantment;

pub trait CombineEnchantments {
    fn combine(
        &self,
        target: &mut Enchantment,
        sacrifice: &Enchantment,
    ) -> CombineEnchantmentsResult;
}

pub type CombineEnchantmentsResult<T = ()> = Result<T, CombineEnchantmentsError>;

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub enum CombineEnchantmentsError {
    EnchantmentKindsIncompatible,
    LevelsIncompatible,
}

impl Display for CombineEnchantmentsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for CombineEnchantmentsError {}
