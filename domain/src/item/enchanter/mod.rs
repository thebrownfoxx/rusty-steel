pub mod agnostic_enchanter;
pub mod compatible_enchantments_enchanter;
pub mod compatible_item_kind_enchanter;
pub mod error;

use crate::enchantment::Enchantment;
use crate::item::Item;
pub use error::{Error, Result};

pub trait Enchanter {
    fn enchant(&self, item: &mut Item, enchantment: Enchantment) -> Result<()>;
}
