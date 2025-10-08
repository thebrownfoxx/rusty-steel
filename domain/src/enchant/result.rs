use crate::combine;
use crate::enchantment::Enchantment;
use std::fmt::{Display, Formatter};
use std::{error, fmt};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub struct Error {
    pub enchantment: Enchantment,
    pub kind: ErrorKind,
}

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub enum ErrorKind {
    ItemKindIncompatible,
    EnchantmentsIncompatible,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl error::Error for Error {}

impl From<combine::enchantments::Error> for Error {
    fn from(value: combine::enchantments::Error) -> Self {
        Error {
            enchantment: value.sacrifice,
            kind: ErrorKind::EnchantmentsIncompatible,
        }
    }
}
