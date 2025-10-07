use super::{
    Combine, RequireSameKind, RetainTarget, UpgradeSameLevel, UpgradeToSacrifice, WithFallback,
    WithMaxLevelCapped,
};
use crate::enchantment::CapMaxLevel;
use std::rc::Rc;

pub fn java(cap_max_level_strategy: Rc<impl CapMaxLevel>) -> impl Combine {
    UpgradeSameLevel
        .with_fallback(UpgradeToSacrifice)
        .with_fallback(RetainTarget)
        .with_max_level_capped(cap_max_level_strategy)
        .require_same_kind()
}
