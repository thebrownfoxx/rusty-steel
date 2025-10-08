mod incompatibility;
mod shared_incompatibility;

pub use incompatibility::IncompatibilityMap;
pub use shared_incompatibility::SharedIncompatibilityMap;
use crate::enchantment::EnchantmentKindId;

pub trait Compatible {
    fn are_compatible(
        &self,
        enchantment_a: &impl AsRef<EnchantmentKindId>,
        enchantment_b: &impl AsRef<EnchantmentKindId>,
    ) -> bool;
}
