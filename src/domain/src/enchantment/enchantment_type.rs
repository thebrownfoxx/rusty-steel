use std::fmt::Debug;
use std::hash::Hash;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug)]
pub struct EnchantmentTypeId(pub String);

pub trait EnchantmentType: Eq + Clone + Hash + Debug {
    fn id(&self) -> &EnchantmentTypeId;
    fn name(&self) -> &str;
    fn max_level(&self) -> u8;
    fn cost_multiplier(&self) -> u8;
}
