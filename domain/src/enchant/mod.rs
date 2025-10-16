mod compatible_enchantments;
mod compatible_item_kind;
mod error;
mod standard;

pub use compatible_enchantments::{CompatibleEnchantments, RequireCompatibleEnchantments};
pub use compatible_item_kind::{CompatibleItemKind, RequireCompatibleItemKind};
pub use error::{Error, Result};

use crate::enchantment::Enchantment;
use crate::item::Item;

pub trait Enchant {
    fn enchant(&self, item: &mut Item, enchantment: Enchantment) -> Result;
}
