mod error;
mod max_level_capped;
mod reject_lower_level_sacrifice;
mod standard;
mod reject_level_overflow;

pub use error::{Error, Result};
pub use max_level_capped::{MaxLevelCapped, MaxLevelCappedCombine};
pub use reject_lower_level_sacrifice::{
    RejectLowerLevelSacrifice, RejectLowerLevelSacrificeCombine,
};
pub use standard::Standard;

use crate::enchantment::Enchantment;

pub trait Combine {
    fn combine(&self, target: &mut Enchantment, sacrifice: &Enchantment) -> Result;
}
