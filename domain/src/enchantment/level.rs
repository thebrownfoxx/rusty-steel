use serde::{Deserialize, Serialize};
use std::cmp::max;

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash, Debug, Serialize, Deserialize)]
pub struct EnchantmentLevel(u8);

impl EnchantmentLevel {
    pub fn new(value: impl Into<u8>) -> Self {
        EnchantmentLevel(value.into())
    }

    pub fn combine(self, other: EnchantmentLevel) -> EnchantmentLevel {
        if self == other {
            return Self::new(self.0 + other.0);
        };

        max(self, other)
    }

    pub fn combine_mut(&mut self, other: EnchantmentLevel) {
        if *self == other {
            self.0 += other.0;
            return;
        };

        self.0 = max(self.0, other.0)
    }
}

impl From<u8> for EnchantmentLevel {
    fn from(value: u8) -> Self {
        Self::new(value)
    }
}

impl From<EnchantmentLevel> for u8 {
    fn from(value: EnchantmentLevel) -> Self {
        value.0
    }
}
