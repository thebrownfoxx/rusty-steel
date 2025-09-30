mod incompatible_enchantment_penalty;
mod zero_enchantment_cost;

use crate::enchantment::Enchantment;
use crate::item::Item;

pub trait EnchantmentCost {
    fn enchantment_cost(&self, target: &Item, enchantment: &Enchantment) -> u8;
}
