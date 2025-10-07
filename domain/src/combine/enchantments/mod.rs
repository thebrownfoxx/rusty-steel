mod bedrock;
mod error;
mod fallback;
mod java;
mod max_level_capped;
mod retain_target;
mod same_kind;
mod upgrade_same_level;
mod upgrade_to_sacrifice;

pub use bedrock::bedrock;
pub use error::{Error, ErrorKind, Result};
pub use fallback::{FallbackStrategy, WithFallback};
pub use java::java;
pub use max_level_capped::{MaxLevelCapped, WithMaxLevelCapped};
pub use retain_target::RetainTarget;
pub use same_kind::{RequireSameKind, SameKind};
pub use upgrade_same_level::UpgradeSameLevel;
pub use upgrade_to_sacrifice::UpgradeToSacrifice;

use crate::enchantment::Enchantment;

pub trait Combine {
    fn combine(&self, target: &mut Enchantment, sacrifice: Enchantment) -> Result;
}
