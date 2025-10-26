use std::fmt::{Display, Formatter};

pub type CombineEnchantmentsResult<T = ()> = Result<T, CombineEnchantmentsError>;

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub enum CombineEnchantmentsError {
    EnchantmentKindsIncompatible,
    LevelsIncompatible,
}

impl Display for CombineEnchantmentsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for CombineEnchantmentsError {}
