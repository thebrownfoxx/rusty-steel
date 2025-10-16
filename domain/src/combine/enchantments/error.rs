use std::fmt::{Display, Formatter};

pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub enum Error {
    EnchantmentKindsIncompatible,
    LevelsIncompatible,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
