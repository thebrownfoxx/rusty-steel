pub mod resolve;

mod agnostic;
mod reject_incompatible_item;
mod resolve_enchantments;
mod standard;

pub use reject_incompatible_item::{RejectIncompatibleItem, RejectIncompatibleItemBuilder};
pub use resolve_enchantments::{IterateEnchantments, IterateEnchantmentsBuilder};
pub use standard::standard_enchanter;

use crate::enchantment::{Enchantment, EnchantmentKindId};
use crate::item::Item;
use std::fmt;
use std::fmt::{Display, Formatter};

pub trait Enchant {
    fn enchant(&self, item: &mut Item, enchantment: Enchantment) -> EnchantResult;
}

