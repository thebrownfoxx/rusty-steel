use domain::enchantment::cost_multiplier::CostMultiplier;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SerializableCostMultiplier {
    pub for_book: u8,
    pub for_item: u8,
}

impl From<SerializableCostMultiplier> for CostMultiplier {
    fn from(value: SerializableCostMultiplier) -> Self {
        CostMultiplier::new(value.for_book, value.for_item)
    }
}

impl From<CostMultiplier> for SerializableCostMultiplier {
    fn from(value: CostMultiplier) -> Self {
        SerializableCostMultiplier {
            for_book: value.for_book,
            for_item: value.for_item,
        }
    }
}
