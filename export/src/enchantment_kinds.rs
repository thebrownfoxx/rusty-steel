use domain::edition::edition_shared::EditionShared;
use domain::enchantment::cost_multiplier::CostMultiplier;
use domain::enchantment::enchantment_kind::EnchantmentKindId;
use domain::enchantment::shared_enchantment_kind::SharedEnchantmentKind;

pub const AQUA_AFFINITY_ID: EnchantmentKindId = EnchantmentKindId(0);
pub const BANE_OF_ARTHROPODS_ID: EnchantmentKindId = EnchantmentKindId(1);
pub const BLAST_PROTECTION_ID: EnchantmentKindId = EnchantmentKindId(2);
pub const BREACH_ID: EnchantmentKindId = EnchantmentKindId(3);
pub const CHANNELING_ID: EnchantmentKindId = EnchantmentKindId(4);
pub const CURSE_OF_BINDING_ID: EnchantmentKindId = EnchantmentKindId(5);
pub const CURSE_OF_VANISHING_ID: EnchantmentKindId = EnchantmentKindId(6);
pub const DENSITY_ID: EnchantmentKindId = EnchantmentKindId(7);
pub const DEPTH_STRIDER_ID: EnchantmentKindId = EnchantmentKindId(8);
pub const EFFICIENCY_ID: EnchantmentKindId = EnchantmentKindId(9);
pub const FEATHER_FALLING_ID: EnchantmentKindId = EnchantmentKindId(10);
pub const FIRE_ASPECT_ID: EnchantmentKindId = EnchantmentKindId(11);
pub const FIRE_PROTECTION_ID: EnchantmentKindId = EnchantmentKindId(12);
pub const FLAME_ID: EnchantmentKindId = EnchantmentKindId(13);
pub const FORTUNE_ID: EnchantmentKindId = EnchantmentKindId(14);
pub const FROST_WALKER_ID: EnchantmentKindId = EnchantmentKindId(15);
pub const IMPALING_ID: EnchantmentKindId = EnchantmentKindId(16);
pub const INFINITY_ID: EnchantmentKindId = EnchantmentKindId(17);
pub const KNOCKBACK_ID: EnchantmentKindId = EnchantmentKindId(18);
pub const LOOTING_ID: EnchantmentKindId = EnchantmentKindId(19);
pub const LOYALTY_ID: EnchantmentKindId = EnchantmentKindId(20);
pub const LUCK_OF_THE_SEA_ID: EnchantmentKindId = EnchantmentKindId(21);
pub const LURE_ID: EnchantmentKindId = EnchantmentKindId(22);
pub const MENDING_ID: EnchantmentKindId = EnchantmentKindId(23);
pub const MULTISHOT_ID: EnchantmentKindId = EnchantmentKindId(24);
pub const PIERCING_ID: EnchantmentKindId = EnchantmentKindId(25);
pub const POWER_ID: EnchantmentKindId = EnchantmentKindId(26);
pub const PROJECTILE_PROTECTION_ID: EnchantmentKindId = EnchantmentKindId(27);
pub const PROTECTION_ID: EnchantmentKindId = EnchantmentKindId(28);
pub const PUNCH_ID: EnchantmentKindId = EnchantmentKindId(29);
pub const QUICK_CHARGE_ID: EnchantmentKindId = EnchantmentKindId(30);
pub const RESPIRATION_ID: EnchantmentKindId = EnchantmentKindId(31);
pub const RIPTIDE_ID: EnchantmentKindId = EnchantmentKindId(32);
pub const SHARPNESS_ID: EnchantmentKindId = EnchantmentKindId(33);
pub const SILK_TOUCH_ID: EnchantmentKindId = EnchantmentKindId(34);
pub const SMITE_ID: EnchantmentKindId = EnchantmentKindId(35);
pub const SOUL_SPEED_ID: EnchantmentKindId = EnchantmentKindId(36);
pub const SWEEPING_EDGE_ID: EnchantmentKindId = EnchantmentKindId(37);
pub const SWIFT_SNEAK_ID: EnchantmentKindId = EnchantmentKindId(38);
pub const THORNS_ID: EnchantmentKindId = EnchantmentKindId(39);
pub const UNBREAKING_ID: EnchantmentKindId = EnchantmentKindId(40);
pub const WIND_BURST_ID: EnchantmentKindId = EnchantmentKindId(41);

pub fn get_enchantment_kinds() -> Vec<SharedEnchantmentKind> {
    vec![
        SharedEnchantmentKind {
            id: AQUA_AFFINITY_ID,
            name: EditionShared::Same("Aqua Affinity".into()),
            max_level: EditionShared::Same(1),
            cost_multiplier: EditionShared::Same(CostMultiplier {
                for_book: 2,
                for_item: 4,
            }),
        },
        SharedEnchantmentKind {
            id: BANE_OF_ARTHROPODS_ID,
            name: EditionShared::Same("Bane of Arthropods".into()),
            max_level: EditionShared::Same(5),
            cost_multiplier: EditionShared::Same(CostMultiplier {
                for_book: 1,
                for_item: 2,
            }),
        },
        SharedEnchantmentKind {
            id: BLAST_PROTECTION_ID,
            name: EditionShared::Same("Blast Protection".into()),
            max_level: EditionShared::Same(4),
            cost_multiplier: EditionShared::Same(CostMultiplier {
                for_book: 2,
                for_item: 4,
            }),
        },
        SharedEnchantmentKind {
            id: BREACH_ID,
            name: EditionShared::Same("Breach".into()),
            max_level: EditionShared::Same(4),
            cost_multiplier: EditionShared::Same(CostMultiplier {
                for_book: 2,
                for_item: 4,
            }),
        },
        SharedEnchantmentKind {
            id: CHANNELING_ID,
            name: EditionShared::Same("Channeling".into()),
            max_level: EditionShared::Same(1),
            cost_multiplier: EditionShared::Same(CostMultiplier {
                for_book: 4,
                for_item: 8,
            }),
        },
        SharedEnchantmentKind {
            id: CURSE_OF_BINDING_ID,
            name: EditionShared::Same("Curse of Binding".into()),
            max_level: EditionShared::Same(1),
            cost_multiplier: EditionShared::Same(CostMultiplier {
                for_book: 4,
                for_item: 8,
            }),
        },
        SharedEnchantmentKind {
            id: CURSE_OF_VANISHING_ID,
            name: EditionShared::Same("Curse of Vanishing".into()),
            max_level: EditionShared::Same(1),
            cost_multiplier: EditionShared::Same(CostMultiplier {
                for_book: 4,
                for_item: 8,
            }),
        },
        SharedEnchantmentKind {
            id: DENSITY_ID,
            name: EditionShared::Same("Density".into()),
            max_level: EditionShared::Same(5),
            cost_multiplier: EditionShared::Same(CostMultiplier {
                for_book: 1,
                for_item: 2,
            }),
        },
        SharedEnchantmentKind {
            id: DEPTH_STRIDER_ID,
            name: EditionShared::Same("Depth Strider".into()),
            max_level: EditionShared::Same(3),
            cost_multiplier: EditionShared::Same(CostMultiplier {
                for_book: 2,
                for_item: 4,
            }),
        },
        SharedEnchantmentKind {
            id: EFFICIENCY_ID,
            name: EditionShared::Same("Efficiency".into()),
            max_level: EditionShared::Same(5),
            cost_multiplier: EditionShared::Same(CostMultiplier {
                for_book: 1,
                for_item: 1,
            }),
        },
        SharedEnchantmentKind {
            id: FEATHER_FALLING_ID,
            name: EditionShared::Same("Feather Falling".into()),
            max_level: EditionShared::Same(4),
            cost_multiplier: EditionShared::Same(CostMultiplier {
                for_book: 1,
                for_item: 2,
            }),
        },
        SharedEnchantmentKind {
            id: FIRE_ASPECT_ID,
            name: EditionShared::Same("Fire Aspect".into()),
            max_level: EditionShared::Same(2),
            cost_multiplier: EditionShared::Same(CostMultiplier {
                for_book: 2,
                for_item: 2,
            }),
        },
        SharedEnchantmentKind {
            id: FIRE_PROTECTION_ID,
            name: EditionShared::Same("Fire Protection".into()),
            max_level: EditionShared::Same(4),
            cost_multiplier: EditionShared::Same(CostMultiplier {
                for_book: 1,
                for_item: 2,
            }),
        },
        SharedEnchantmentKind {
            id: FLAME_ID,
            name: EditionShared::Same("Flame".into()),
            max_level: EditionShared::Same(1),
            cost_multiplier: EditionShared::Same(CostMultiplier {
                for_book: 2,
                for_item: 4,
            }),
        },
        SharedEnchantmentKind {
            id: FORTUNE_ID,
            name: EditionShared::Same("Fortune".into()),
            max_level: EditionShared::Same(3),
            cost_multiplier: EditionShared::Same(CostMultiplier {
                for_book: 2,
                for_item: 4,
            }),
        },
        SharedEnchantmentKind {
            id: FROST_WALKER_ID,
            name: EditionShared::Same("Frost Walker".into()),
            max_level: EditionShared::Same(2),
            cost_multiplier: EditionShared::Same(CostMultiplier {
                for_book: 2,
                for_item: 4,
            }),
        },
        SharedEnchantmentKind {
            id: IMPALING_ID,
            name: EditionShared::Same("Impaling".into()),
            max_level: EditionShared::Same(5),
            cost_multiplier: EditionShared::Different {
                for_bedrock: CostMultiplier {
                    for_book: 2,
                    for_item: 1,
                },
                for_java: CostMultiplier {
                    for_book: 2,
                    for_item: 4,
                },
            },
        },
        SharedEnchantmentKind {
            id: INFINITY_ID,
            name: EditionShared::Same("Infinity".into()),
            max_level: EditionShared::Same(1),
            cost_multiplier: EditionShared::Same(CostMultiplier {
                for_book: 4,
                for_item: 8,
            }),
        },
        SharedEnchantmentKind {
            id: KNOCKBACK_ID,
            name: EditionShared::Same("Knockback".into()),
            max_level: EditionShared::Same(2),
            cost_multiplier: EditionShared::Same(CostMultiplier {
                for_book: 1,
                for_item: 2,
            }),
        },
        SharedEnchantmentKind {
            id: LOOTING_ID,
            name: EditionShared::Same("Looting".into()),
            max_level: EditionShared::Same(3),
            cost_multiplier: EditionShared::Same(CostMultiplier {
                for_book: 2,
                for_item: 4,
            }),
        },
        SharedEnchantmentKind {
            id: LOYALTY_ID,
            name: EditionShared::Same("Loyalty".into()),
            max_level: EditionShared::Same(3),
            cost_multiplier: EditionShared::Same(CostMultiplier {
                for_book: 1,
                for_item: 1,
            }),
        },
        SharedEnchantmentKind {
            id: LUCK_OF_THE_SEA_ID,
            name: EditionShared::Same("Luck of the Sea".into()),
            max_level: EditionShared::Same(3),
            cost_multiplier: EditionShared::Same(CostMultiplier {
                for_book: 2,
                for_item: 4,
            }),
        },
        SharedEnchantmentKind {
            id: LURE_ID,
            name: EditionShared::Same("Lure".into()),
            max_level: EditionShared::Same(3),
            cost_multiplier: EditionShared::Same(CostMultiplier {
                for_book: 2,
                for_item: 4,
            }),
        },
        SharedEnchantmentKind {
            id: MENDING_ID,
            name: EditionShared::Same("Mending".into()),
            max_level: EditionShared::Same(1),
            cost_multiplier: EditionShared::Same(CostMultiplier {
                for_book: 2,
                for_item: 4,
            }),
        },
        SharedEnchantmentKind {
            id: MULTISHOT_ID,
            name: EditionShared::Same("Multishot".into()),
            max_level: EditionShared::Same(1),
            cost_multiplier: EditionShared::Same(CostMultiplier {
                for_book: 2,
                for_item: 4,
            }),
        },
        SharedEnchantmentKind {
            id: PIERCING_ID,
            name: EditionShared::Same("Piercing".into()),
            max_level: EditionShared::Same(4),
            cost_multiplier: EditionShared::Same(CostMultiplier {
                for_book: 1,
                for_item: 1,
            }),
        },
        SharedEnchantmentKind {
            id: POWER_ID,
            name: EditionShared::Same("Power".into()),
            max_level: EditionShared::Same(5),
            cost_multiplier: EditionShared::Same(CostMultiplier {
                for_book: 1,
                for_item: 1,
            }),
        },
        SharedEnchantmentKind {
            id: PROJECTILE_PROTECTION_ID,
            name: EditionShared::Same("Projectile Protection".into()),
            max_level: EditionShared::Same(4),
            cost_multiplier: EditionShared::Same(CostMultiplier {
                for_book: 1,
                for_item: 2,
            }),
        },
        SharedEnchantmentKind {
            id: PROTECTION_ID,
            name: EditionShared::Same("Protection".into()),
            max_level: EditionShared::Same(4),
            cost_multiplier: EditionShared::Same(CostMultiplier {
                for_book: 1,
                for_item: 1,
            }),
        },
        SharedEnchantmentKind {
            id: PUNCH_ID,
            name: EditionShared::Same("Punch".into()),
            max_level: EditionShared::Same(2),
            cost_multiplier: EditionShared::Same(CostMultiplier {
                for_book: 2,
                for_item: 4,
            }),
        },
        SharedEnchantmentKind {
            id: QUICK_CHARGE_ID,
            name: EditionShared::Same("Quick Charge".into()),
            max_level: EditionShared::Same(3),
            cost_multiplier: EditionShared::Same(CostMultiplier {
                for_book: 1,
                for_item: 2,
            }),
        },
        SharedEnchantmentKind {
            id: RESPIRATION_ID,
            name: EditionShared::Same("Respiration".into()),
            max_level: EditionShared::Same(3),
            cost_multiplier: EditionShared::Same(CostMultiplier {
                for_book: 2,
                for_item: 4,
            }),
        },
        SharedEnchantmentKind {
            id: RIPTIDE_ID,
            name: EditionShared::Same("Riptide".into()),
            max_level: EditionShared::Same(3),
            cost_multiplier: EditionShared::Same(CostMultiplier {
                for_book: 2,
                for_item: 4,
            }),
        },
        SharedEnchantmentKind {
            id: SHARPNESS_ID,
            name: EditionShared::Same("Sharpness".into()),
            max_level: EditionShared::Same(5),
            cost_multiplier: EditionShared::Same(CostMultiplier {
                for_book: 1,
                for_item: 1,
            }),
        },
        SharedEnchantmentKind {
            id: SILK_TOUCH_ID,
            name: EditionShared::Same("Silk Touch".into()),
            max_level: EditionShared::Same(1),
            cost_multiplier: EditionShared::Same(CostMultiplier {
                for_book: 4,
                for_item: 8,
            }),
        },
        SharedEnchantmentKind {
            id: SMITE_ID,
            name: EditionShared::Same("Smite".into()),
            max_level: EditionShared::Same(5),
            cost_multiplier: EditionShared::Same(CostMultiplier {
                for_book: 1,
                for_item: 2,
            }),
        },
        SharedEnchantmentKind {
            id: SOUL_SPEED_ID,
            name: EditionShared::Same("Soul Speed".into()),
            max_level: EditionShared::Same(3),
            cost_multiplier: EditionShared::Same(CostMultiplier {
                for_book: 4,
                for_item: 8,
            }),
        },
        SharedEnchantmentKind {
            id: SWEEPING_EDGE_ID,
            name: EditionShared::Different {
                for_java: "Sweeping Edge".into(),
                for_bedrock: "".into(),
            },
            max_level: EditionShared::Different {
                for_java: 3,
                for_bedrock: 0,
            },
            cost_multiplier: EditionShared::Different {
                for_bedrock: CostMultiplier {
                    for_book: 0,
                    for_item: 0,
                },
                for_java: CostMultiplier {
                    for_book: 2,
                    for_item: 4,
                },
            },
        },
        SharedEnchantmentKind {
            id: SWIFT_SNEAK_ID,
            name: EditionShared::Same("Swift Sneak".into()),
            max_level: EditionShared::Same(3),
            cost_multiplier: EditionShared::Same(CostMultiplier {
                for_book: 4,
                for_item: 8,
            }),
        },
        SharedEnchantmentKind {
            id: THORNS_ID,
            name: EditionShared::Same("Thorns".into()),
            max_level: EditionShared::Same(3),
            cost_multiplier: EditionShared::Same(CostMultiplier {
                for_book: 4,
                for_item: 8,
            }),
        },
        SharedEnchantmentKind {
            id: UNBREAKING_ID,
            name: EditionShared::Same("Unbreaking".into()),
            max_level: EditionShared::Same(3),
            cost_multiplier: EditionShared::Same(CostMultiplier {
                for_book: 1,
                for_item: 2,
            }),
        },
        SharedEnchantmentKind {
            id: WIND_BURST_ID,
            name: EditionShared::Same("Wind Burst".into()),
            max_level: EditionShared::Same(3),
            cost_multiplier: EditionShared::Same(CostMultiplier {
                for_book: 2,
                for_item: 4,
            }),
        },
    ]
}
