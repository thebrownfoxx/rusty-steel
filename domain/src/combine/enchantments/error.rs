use std::fmt::{Display, Formatter};
use crate::enchantment::Enchantment;

pub type Result = std::result::Result<(), Error>;

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub struct Error {
    pub sacrifice: Enchantment,
    pub kind: ErrorKind,
}

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub enum ErrorKind {
    IncompatibleEnchantments,
    IncompatibleLevels,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
