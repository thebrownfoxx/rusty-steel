mod agnostic_enchant;
mod compatible_enchantments_enchant;
mod compatible_item_kind_enchant;
mod error;

pub use agnostic_enchant::AgnosticEnchant;
pub use compatible_enchantments_enchant::{
    CompatibleEnchantmentsEnchant, RequireCompatibleEnchantments,
};
pub use compatible_item_kind_enchant::{CompatibleItemKindEnchant, RequireCompatibleItemKind};
pub use error::{Error, Result};

use crate::enchantment::Enchantment;
use crate::item::Item;

pub trait Enchant {
    fn enchant(&self, item: &mut Item, enchantment: Enchantment) -> Result<()>;
}
