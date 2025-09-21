use crate::compatibility_entry::entry;
use crate::enchantment_kind_ids::EnchantmentKindIds;
use domain::edition::EditionShared;
use domain::enchantment::EnchantmentKindId;
use std::collections::BTreeMap;

pub fn get_enchantment_incompatibility_matrix(
    ids: &EnchantmentKindIds,
) -> BTreeMap<EnchantmentKindId, EditionShared<Vec<EnchantmentKindId>>> {
    BTreeMap::from([
        entry![ids.aqua_affinity],
        entry![ids.bane_of_arthropods => ids.sharpness, ids.smite],
        entry![
            ids.blast_protection =>
            ids.fire_protection, ids.projectile_protection, ids.protection
        ],
        entry![ids.breach => ids.bane_of_arthropods, ids.density, ids.smite],
        entry![ids.channeling => ids.riptide],
        entry![ids.curse_of_binding],
        entry![ids.curse_of_vanishing],
        entry![ids.density => ids.bane_of_arthropods, ids.breach, ids.smite],
        entry![ids.depth_strider => ids.frost_walker],
        entry![ids.efficiency],
        entry![ids.feather_falling],
        entry![ids.fire_aspect],
        entry![
            ids.fire_protection =>
            ids.blast_protection, ids.projectile_protection, ids.protection
        ],
        entry![ids.flame],
        entry![ids.fortune => ids.silk_touch],
        entry![ids.frost_walker => ids.depth_strider],
        entry![ids.impaling],
        entry![ids.infinity => ids.mending],
        entry![ids.knockback],
        entry![ids.looting],
        entry![ids.loyalty => ids.riptide],
        entry![ids.luck_of_the_sea],
        entry![ids.lure],
        entry![ids.mending => ids.infinity],
        entry![ids.multishot => ids.piercing],
        entry![ids.piercing => ids.multishot],
        entry![ids.power],
        entry![
            ids.projectile_protection =>
            ids.blast_protection, ids.fire_protection, ids.protection
        ],
        entry![
            ids.protection =>
            ids.blast_protection, ids.fire_protection, ids.projectile_protection
        ],
        entry![ids.punch],
        entry![ids.quick_charge],
        entry![ids.respiration],
        entry![ids.riptide => ids.channeling, ids.loyalty],
        entry![ids.sharpness => ids.bane_of_arthropods, ids.smite],
        entry![ids.silk_touch => ids.fortune],
        entry![ids.smite => ids.bane_of_arthropods, ids.sharpness],
        entry![ids.soul_speed],
        entry![ids.sweeping_edge],
        entry![ids.swift_sneak],
        entry![ids.thorns],
        entry![ids.unbreaking],
        entry![ids.wind_burst],
    ])
}
