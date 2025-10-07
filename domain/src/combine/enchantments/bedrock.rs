use super::{Combine, RequireSameKind, UpgradeToSacrifice, WithFallback, WithMaxLevelCapped};
use crate::enchantment::CapMaxLevel;
use std::rc::Rc;

pub fn bedrock(cap_max_level_strategy: Rc<impl CapMaxLevel>) -> impl Combine {
    UpgradeToSacrifice
        .with_fallback(UpgradeToSacrifice)
        .with_max_level_capped(cap_max_level_strategy)
        .require_same_kind()
}
