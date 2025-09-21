use std::fmt::{Display, Formatter};

pub type Result<T> = std::result::Result<T, crate::enchant::Error>;

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash, Debug)]
pub enum Error {
    ItemKindsIncompatible,
    NoCompatibleEnchantments,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
