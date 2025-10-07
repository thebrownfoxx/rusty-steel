mod compatibility;
mod shared_compatibility;

use crate::enchantment::EnchantmentKindId;
use crate::item::ItemKindId;
pub use compatibility::EnchantmentCompatibilityMap;
pub use shared_compatibility::SharedEnchantmentCompatibilityMap;

pub trait AreCompatible {
    fn are_compatible(
        &self,
        item: &dyn AsRef<ItemKindId>,
        enchantment: &dyn AsRef<EnchantmentKindId>,
    ) -> bool;
}
