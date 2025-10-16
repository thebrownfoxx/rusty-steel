use crate::enchantment::EnchantmentKindId;
use std::fmt::{Display, Formatter};
use std::{error, fmt};

pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub enum Error {
    ItemKindIncompatible,
    EnchantmentsIncompatible { conflict: EnchantmentKindId },
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl error::Error for Error {}
