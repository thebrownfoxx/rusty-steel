use crate::enchantment::{Enchantment, EnchantmentKindId, EnchantmentLevel};
use crate::item::Item;
use std::fmt;
use std::fmt::{Display, Formatter};

pub struct EnchantItem;

impl EnchantItem {
    pub fn basic(
        item: &mut Item,
        enchantment: Enchantment,
        combine_enchantments: Fn(
            &EnchantmentKindId,
            EnchantmentLevel,
            EnchantmentLevel,
        ) -> Option<EnchantmentLevel>,
    ) -> EnchantItemResult {
    }
}

pub type EnchantItemResult<T = ()> = Result<T, EnchantItemError>;

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub enum EnchantItemError {
    IncompatibleItemKind,
    IncompatibleEnchantments { conflict: EnchantmentKindId },
}

impl Display for EnchantItemError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for EnchantItemError {}
