mod enchant;
mod merge;
mod reject_incompatible;
mod standard;
mod unhandled;

pub use enchant::{ResolveEnchantmentsEnchant, ResolveEnchantmentsEnchanter};
pub use merge::{MergeEnchantmentResolver, MergeResolveEnchantments};
pub use reject_incompatible::{
    RejectIncompatibleEnchantmentResolver, RejectIncompatibleResolveEnchantments,
};
pub use standard::standard_enchantment_resolver;
pub use unhandled::UnhandledEnchantmentResolver;

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
