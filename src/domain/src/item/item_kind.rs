use crate::enchantment::enchantment_kind::EnchantmentKindId;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug)]
pub struct ItemKindId(pub String);

pub trait ItemKind: Eq + Clone + Hash + Debug {
    fn id(&self) -> &ItemKindId;
    fn name(&self) -> &str;
    fn compatible_enchantments(&self) -> impl Iterator<Item = &EnchantmentKindId>;
}
