pub mod agnostic_enchant;
pub mod compatible_enchantments_enchant;
pub mod compatible_item_kind_enchant;
pub mod error;

use crate::enchantment::Enchantment;
use crate::item::Item;
pub use error::{Error, Result};

pub trait Enchant {
    fn enchant(&self, item: &mut Item, enchantment: Enchantment) -> Result<()>;
}
