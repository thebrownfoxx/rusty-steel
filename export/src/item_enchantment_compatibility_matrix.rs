use crate::enchantment_kind_ids::EnchantmentKindIds;
use crate::item_kind_ids::ItemKindIds;
use domain::edition::EditionShared;
use domain::enchantment::EnchantmentKindId;
use domain::item::ItemKindId;
use std::collections::BTreeMap;
use crate::compatibility_entry::entry;

pub fn get_item_enchantment_compatibility_matrix(
    item_ids: &ItemKindIds,
    enchantment_ids: &EnchantmentKindIds,
) -> BTreeMap<ItemKindId, EditionShared<Vec<EnchantmentKindId>>> {
    BTreeMap::from([
        (
            item_ids.enchanted_book.clone(),
            enchantment_ids.all().into(),
        ),
        entry![
            item_ids.helmet =>
            enchantment_ids.aqua_affinity,
            enchantment_ids.blast_protection,
            enchantment_ids.curse_of_binding,
            enchantment_ids.curse_of_vanishing,
            enchantment_ids.fire_protection,
            enchantment_ids.mending,
            enchantment_ids.projectile_protection,
            enchantment_ids.protection,
            enchantment_ids.respiration,
            enchantment_ids.thorns,
            enchantment_ids.unbreaking,
        ],
        entry![
            item_ids.chestplate =>
            enchantment_ids.blast_protection,
            enchantment_ids.curse_of_binding,
            enchantment_ids.curse_of_vanishing,
            enchantment_ids.fire_protection,
            enchantment_ids.mending,
            enchantment_ids.projectile_protection,
            enchantment_ids.protection,
            enchantment_ids.thorns,
            enchantment_ids.unbreaking,
        ],
        entry![
            item_ids.leggings =>
            enchantment_ids.blast_protection,
            enchantment_ids.curse_of_binding,
            enchantment_ids.curse_of_vanishing,
            enchantment_ids.fire_protection,
            enchantment_ids.mending,
            enchantment_ids.projectile_protection,
            enchantment_ids.protection,
            enchantment_ids.swift_sneak,
            enchantment_ids.thorns,
            enchantment_ids.unbreaking,
        ],
        entry![
            item_ids.boots =>
            enchantment_ids.blast_protection,
            enchantment_ids.curse_of_binding,
            enchantment_ids.curse_of_vanishing,
            enchantment_ids.depth_strider,
            enchantment_ids.feather_falling,
            enchantment_ids.fire_protection,
            enchantment_ids.frost_walker,
            enchantment_ids.mending,
            enchantment_ids.projectile_protection,
            enchantment_ids.protection,
            enchantment_ids.soul_speed,
            enchantment_ids.thorns,
            enchantment_ids.unbreaking,
        ],
        entry![
            item_ids.elytra =>
            enchantment_ids.curse_of_binding,
            enchantment_ids.curse_of_vanishing,
            enchantment_ids.mending,
            enchantment_ids.unbreaking,
        ],
        entry![
            item_ids.head =>
            enchantment_ids.curse_of_binding,
            enchantment_ids.curse_of_vanishing,
        ],
        entry![
            item_ids.sword =>
            enchantment_ids.bane_of_arthropods,
            enchantment_ids.curse_of_vanishing,
            enchantment_ids.fire_aspect,
            enchantment_ids.knockback,
            enchantment_ids.looting,
            enchantment_ids.mending,
            enchantment_ids.sharpness,
            enchantment_ids.smite,
            enchantment_ids.sweeping_edge,
            enchantment_ids.unbreaking,
        ],
        entry![
            item_ids.mace =>
            enchantment_ids.density,
            enchantment_ids.breach,
            enchantment_ids.wind_burst,
            enchantment_ids.smite,
            enchantment_ids.bane_of_arthropods,
            enchantment_ids.fire_aspect,
            enchantment_ids.unbreaking,
            enchantment_ids.mending,
            enchantment_ids.curse_of_vanishing,
        ],
        entry![
            item_ids.axe =>
            enchantment_ids.bane_of_arthropods,
            enchantment_ids.curse_of_vanishing,
            enchantment_ids.efficiency,
            enchantment_ids.fortune,
            enchantment_ids.mending,
            enchantment_ids.sharpness,
            enchantment_ids.silk_touch,
            enchantment_ids.smite,
            enchantment_ids.unbreaking,
        ],
        entry![
            item_ids.pickaxe =>
            enchantment_ids.curse_of_vanishing,
            enchantment_ids.efficiency,
            enchantment_ids.fortune,
            enchantment_ids.mending,
            enchantment_ids.silk_touch,
            enchantment_ids.unbreaking,
        ],
        entry![
            item_ids.shovel =>
            enchantment_ids.curse_of_vanishing,
            enchantment_ids.efficiency,
            enchantment_ids.fortune,
            enchantment_ids.mending,
            enchantment_ids.silk_touch,
            enchantment_ids.unbreaking,
        ],
        entry![
            item_ids.hoe =>
            enchantment_ids.curse_of_vanishing,
            enchantment_ids.efficiency,
            enchantment_ids.fortune,
            enchantment_ids.mending,
            enchantment_ids.silk_touch,
            enchantment_ids.unbreaking,
        ],
        entry![
            item_ids.bow =>
            enchantment_ids.curse_of_vanishing,
            enchantment_ids.flame,
            enchantment_ids.infinity,
            enchantment_ids.mending,
            enchantment_ids.power,
            enchantment_ids.punch,
            enchantment_ids.unbreaking,
        ],
        entry![
            item_ids.fishing_rod =>
            enchantment_ids.curse_of_vanishing,
            enchantment_ids.luck_of_the_sea,
            enchantment_ids.lure,
            enchantment_ids.mending,
            enchantment_ids.unbreaking,
        ],
        entry![
            item_ids.trident =>
            enchantment_ids.channeling,
            enchantment_ids.curse_of_vanishing,
            enchantment_ids.impaling,
            enchantment_ids.loyalty,
            enchantment_ids.mending,
            enchantment_ids.riptide,
            enchantment_ids.unbreaking,
        ],
        entry![
            item_ids.crossbow =>
            enchantment_ids.curse_of_vanishing,
            enchantment_ids.mending,
            enchantment_ids.multishot,
            enchantment_ids.piercing,
            enchantment_ids.quick_charge,
            enchantment_ids.unbreaking,
        ],
        entry![
            item_ids.shears =>
            enchantment_ids.curse_of_vanishing,
            enchantment_ids.efficiency,
            enchantment_ids.mending,
            enchantment_ids.unbreaking,
        ],
        entry![
            item_ids.flint_and_steel =>
            enchantment_ids.curse_of_vanishing,
            enchantment_ids.mending,
            enchantment_ids.unbreaking,
        ],
        entry![
            item_ids.carrot_on_a_stick =>
            enchantment_ids.curse_of_vanishing,
            enchantment_ids.mending,
            enchantment_ids.unbreaking,
        ],
        entry![
            item_ids.warped_fungus_on_a_stick =>
            enchantment_ids.curse_of_vanishing,
            enchantment_ids.mending,
            enchantment_ids.unbreaking,
        ],
    ])
}
