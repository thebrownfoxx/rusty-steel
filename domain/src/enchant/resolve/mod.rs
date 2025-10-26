mod compatible;
mod enchant;
mod merge;
mod standard;
mod unhandled;

pub use compatible::{RequireCompatible, RequireCompatibleResolveEnchantments};
pub use enchant::{ResolveAgainstTargetEnchantments, ResolvingEnchant};
pub use merge::{Merge, MergeResolveEnchantments};
pub use standard::standard_resolver;
pub use unhandled::UnhandledResolveEnchantments;

use crate::enchantment::{Enchantment, EnchantmentKindId};

pub trait ResolveEnchantments {
    fn resolve(&self, target: &mut Enchantment, sacrifice: &Enchantment) -> EnchantmentsResolution;
}

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub enum EnchantmentsResolution {
    Handled,
    Unhandled,
    Incompatible { conflict: EnchantmentKindId },
}
