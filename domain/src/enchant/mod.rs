mod agnostic;
mod compatible_enchantments;
mod compatible_item_kind;
mod enchantments_combined;
mod result;
mod fallback;
mod standard;

pub use agnostic::Agnostic;
pub use compatible_enchantments::{CompatibleEnchantments, RequireCompatibleEnchantments};
pub use compatible_item_kind::{CompatibleItemKind, RequireCompatibleItemKind};
pub use enchantments_combined::EnchantmentsCombined;
pub use result::{Error, ErrorKind, Result};
pub use fallback::{FallbackStrategy, WithFallback};

use crate::enchantment::Enchantment;
use crate::item::Item;

pub trait Enchant {
    fn enchant(&self, item: &mut Item, enchantment: Enchantment) -> Result<()>;
}
