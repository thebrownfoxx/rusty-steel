use serde::{Deserialize, Serialize};
use std::ops::Add;

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash, Debug, Serialize, Deserialize)]
pub struct EnchantmentLevel(u8);

impl EnchantmentLevel {
    pub fn new(value: impl Into<u8>) -> Self {
        Self(value.into())
    }

    pub fn into_u8(self) -> u8 {
        self.0
    }
}

impl From<u8> for EnchantmentLevel {
    fn from(value: u8) -> Self {
        Self::new(value)
    }
}

impl From<EnchantmentLevel> for u8 {
    fn from(value: EnchantmentLevel) -> Self {
        value.into_u8()
    }
}

impl<T: Into<EnchantmentLevel>> Add<T> for EnchantmentLevel {
    type Output = EnchantmentLevel;

    fn add(self, rhs: T) -> Self::Output {
        Self(self.into_u8() + rhs.into().into_u8())
    }
}
