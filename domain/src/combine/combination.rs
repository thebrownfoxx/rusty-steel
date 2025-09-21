use bon::Builder;
use serde::{Deserialize, Serialize};
use crate::item::Item;

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, Builder)]
pub struct Combination {
    #[builder(into)]
    pub result: Item,
    pub cost: u8,
}
