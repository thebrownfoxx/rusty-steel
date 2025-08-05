use crate::enchantment::enchantment_type::EnchantmentTypeId;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug)]
pub struct ItemTypeId(pub String);

pub trait ItemType: Eq + Clone + Hash + Debug {
    fn id(&self) -> &ItemTypeId;
    fn name(&self) -> &str;
    fn compatible_enchantments(&self) -> impl Iterator<Item = &EnchantmentTypeId>;
}
