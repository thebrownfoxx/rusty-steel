use crate::enchantment_kinds::*;
use domain::edition::EditionShared;
use domain::enchantment::EnchantmentKindId;
use std::collections::BTreeMap;

pub fn get_enchantment_incompatibility_matrix()
-> BTreeMap<EnchantmentKindId, EditionShared<Vec<EnchantmentKindId>>> {
    BTreeMap::from([
        (AQUA_AFFINITY_ID, EditionShared::Same(vec![])),
        (
            BANE_OF_ARTHROPODS_ID,
            EditionShared::Same(vec![SHARPNESS_ID, SMITE_ID]),
        ),
        (
            BLAST_PROTECTION_ID,
            EditionShared::Same(vec![
                FIRE_PROTECTION_ID,
                PROJECTILE_PROTECTION_ID,
                PROTECTION_ID,
            ]),
        ),
        (
            BREACH_ID,
            EditionShared::Same(vec![BANE_OF_ARTHROPODS_ID, DENSITY_ID, SMITE_ID]),
        ),
        (CHANNELING_ID, EditionShared::Same(vec![RIPTIDE_ID])),
        (CURSE_OF_BINDING_ID, EditionShared::Same(vec![])),
        (CURSE_OF_VANISHING_ID, EditionShared::Same(vec![])),
        (
            DENSITY_ID,
            EditionShared::Same(vec![BANE_OF_ARTHROPODS_ID, BREACH_ID, SMITE_ID]),
        ),
        (DEPTH_STRIDER_ID, EditionShared::Same(vec![FROST_WALKER_ID])),
        (EFFICIENCY_ID, EditionShared::Same(vec![])),
        (FEATHER_FALLING_ID, EditionShared::Same(vec![])),
        (FIRE_ASPECT_ID, EditionShared::Same(vec![])),
        (
            FIRE_PROTECTION_ID,
            EditionShared::Same(vec![
                BLAST_PROTECTION_ID,
                PROJECTILE_PROTECTION_ID,
                PROTECTION_ID,
            ]),
        ),
        (FLAME_ID, EditionShared::Same(vec![])),
        (FORTUNE_ID, EditionShared::Same(vec![SILK_TOUCH_ID])),
        (FROST_WALKER_ID, EditionShared::Same(vec![DEPTH_STRIDER_ID])),
        (IMPALING_ID, EditionShared::Same(vec![])),
        (INFINITY_ID, EditionShared::Same(vec![MENDING_ID])),
        (KNOCKBACK_ID, EditionShared::Same(vec![])),
        (LOOTING_ID, EditionShared::Same(vec![])),
        (LOYALTY_ID, EditionShared::Same(vec![RIPTIDE_ID])),
        (LUCK_OF_THE_SEA_ID, EditionShared::Same(vec![])),
        (LURE_ID, EditionShared::Same(vec![])),
        (MENDING_ID, EditionShared::Same(vec![INFINITY_ID])),
        (MULTISHOT_ID, EditionShared::Same(vec![PIERCING_ID])),
        (PIERCING_ID, EditionShared::Same(vec![MULTISHOT_ID])),
        (POWER_ID, EditionShared::Same(vec![])),
        (
            PROJECTILE_PROTECTION_ID,
            EditionShared::Same(vec![BLAST_PROTECTION_ID, FIRE_PROTECTION_ID, PROTECTION_ID]),
        ),
        (
            PROTECTION_ID,
            EditionShared::Same(vec![
                BLAST_PROTECTION_ID,
                FIRE_PROTECTION_ID,
                PROJECTILE_PROTECTION_ID,
            ]),
        ),
        (PUNCH_ID, EditionShared::Same(vec![])),
        (QUICK_CHARGE_ID, EditionShared::Same(vec![])),
        (RESPIRATION_ID, EditionShared::Same(vec![])),
        (
            RIPTIDE_ID,
            EditionShared::Same(vec![CHANNELING_ID, LOYALTY_ID]),
        ),
        (
            SHARPNESS_ID,
            EditionShared::Same(vec![BANE_OF_ARTHROPODS_ID, SMITE_ID]),
        ),
        (SILK_TOUCH_ID, EditionShared::Same(vec![FORTUNE_ID])),
        (
            SMITE_ID,
            EditionShared::Same(vec![BANE_OF_ARTHROPODS_ID, SHARPNESS_ID]),
        ),
        (SOUL_SPEED_ID, EditionShared::Same(vec![])),
        (SWEEPING_EDGE_ID, EditionShared::Same(vec![])),
        (SWIFT_SNEAK_ID, EditionShared::Same(vec![])),
        (THORNS_ID, EditionShared::Same(vec![])),
        (UNBREAKING_ID, EditionShared::Same(vec![])),
        (WIND_BURST_ID, EditionShared::Same(vec![])),
    ])
}
