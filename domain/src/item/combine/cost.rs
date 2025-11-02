pub struct CombineCost(u8);

impl CombineCost {
    pub fn new(value: impl Into<u8>) -> Self {
        Self(value.into())
    }
}

impl From<u8> for CombineCost {
    fn from(value: u8) -> Self {
        Self::new(value)
    }
}

impl From<CombineCost> for u8 {
    fn from(value: CombineCost) -> Self {
        value.0
    }
}
