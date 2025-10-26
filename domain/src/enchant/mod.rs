pub mod resolve;

mod agnostic;
mod require_compatible_item;
mod standard;

pub use require_compatible_item::{RequireCompatibleItemEnchant, RequireCompatibleItemEnchanter};
pub use standard::standard_enchanter;

use crate::enchantment::{Enchantment, EnchantmentKindId};
use crate::item::Item;
use std::fmt;
use std::fmt::{Display, Formatter};

pub trait Enchant {
    fn enchant(&self, item: &mut Item, enchantment: Enchantment) -> EnchantResult;
}

pub type EnchantResult<T = ()> = Result<T, EnchantError>;

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub enum EnchantError {
    ItemKindIncompatible,
    EnchantmentsIncompatible { conflict: EnchantmentKindId },
}

impl Display for EnchantError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for EnchantError {}
