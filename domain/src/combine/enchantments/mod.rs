mod error;
mod max_level_capped;
mod reject_lower_level_sacrifice;
mod standard;

pub use error::{Error, Result};
pub use max_level_capped::MaxLevelCapped;
pub use reject_lower_level_sacrifice::RejectLowerLevelSacrifice;

use crate::enchantment::Enchantment;

pub trait Combine {
    fn combine(&self, target: &mut Enchantment, sacrifice: &Enchantment) -> Result;
}
