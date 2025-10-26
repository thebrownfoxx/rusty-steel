mod error;
mod max_level_capped;
mod reject_level_overflow;
mod reject_lower_level_sacrifice;
mod standard;

pub use error::{CombineEnchantmentsError, CombineEnchantmentsResult};
pub use max_level_capped::{MaxLevelCapped, MaxLevelCappedCombineEnchantments};
pub use reject_level_overflow::{RejectLevelOverflow, RejectLevelOverflowCombineEnchantments};
pub use reject_lower_level_sacrifice::{
    RejectLowerLevelSacrifice, RejectLowerLevelSacrificeCombineEnchantments,
};
pub use standard::StandardCombineEnchantments;

use crate::enchantment::Enchantment;

pub trait CombineEnchantments {
    fn combine(
        &self,
        target: &mut Enchantment,
        sacrifice: &Enchantment,
    ) -> CombineEnchantmentsResult;
}
