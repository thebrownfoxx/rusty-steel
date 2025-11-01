mod merge;
mod reject_incompatible;
mod standard;
mod unhandled;

pub use merge::{Merge, MergeBuilder};
pub use reject_incompatible::{RejectIncompatible, RejectIncompatibleBuilder};
pub use standard::standard_enchantment_resolver;
pub use unhandled::UnhandledEnchantmentsResolver;

use crate::enchantment::Enchantment;

pub trait ResolveEnchantments {
    fn resolve(&self, target: &mut Enchantment, sacrifice: &Enchantment) -> EnchantmentsResolution;
}

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub enum EnchantmentsResolution {
    Resolved,
    Unresolved,
    Incompatible,
}
