use crate::edition::Edition;
use crate::enchantment::cost_multiplier::CostMultiplier;
use crate::enchantment::shared::shared_enchantment_type::SharedEnchantmentType;
use crate::enchantment::shared::shared_enchantment_types::SharedEnchantmentTypes;

pub fn hardcoded_enchantment_types() -> impl SharedEnchantmentTypes {
    vec![
        SharedEnchantmentType::new(
            "aqua_affinity",
            "Aqua Affinity",
            1,
            Box::new(|_edition| Some(CostMultiplier { item: 4, book: 2 })),
        ),
        SharedEnchantmentType::new(
            "bane_of_arthropods",
            "Bane of Arthropods",
            5,
            Box::new(|_edition| Some(CostMultiplier { item: 2, book: 1 })),
        ),
        SharedEnchantmentType::new(
            "blast_protection",
            "Blast Protection",
            4,
            Box::new(|_edition| Some(CostMultiplier { item: 4, book: 2 })),
        ),
        SharedEnchantmentType::new(
            "breach",
            "Breach",
            4,
            Box::new(|_edition| Some(CostMultiplier { item: 4, book: 2 })),
        ),
        SharedEnchantmentType::new(
            "channeling",
            "Channeling",
            1,
            Box::new(|_edition| Some(CostMultiplier { item: 8, book: 4 })),
        ),
        SharedEnchantmentType::new(
            "curse_of_binding",
            "Curse of Binding",
            1,
            Box::new(|_edition| Some(CostMultiplier { item: 8, book: 4 })),
        ),
        SharedEnchantmentType::new(
            "curse_of_vanishing",
            "Curse of Vanishing",
            1,
            Box::new(|_edition| Some(CostMultiplier { item: 8, book: 4 })),
        ),
        SharedEnchantmentType::new(
            "density",
            "Density",
            5,
            Box::new(|_edition| Some(CostMultiplier { item: 2, book: 1 })),
        ),
        SharedEnchantmentType::new(
            "depth_strider",
            "Depth Strider",
            3,
            Box::new(|_edition| Some(CostMultiplier { item: 4, book: 2 })),
        ),
        SharedEnchantmentType::new(
            "efficiency",
            "Efficiency",
            5,
            Box::new(|_edition| Some(CostMultiplier { item: 1, book: 1 })),
        ),
        SharedEnchantmentType::new(
            "feather_falling",
            "Feather Falling",
            4,
            Box::new(|_edition| Some(CostMultiplier { item: 2, book: 1 })),
        ),
        SharedEnchantmentType::new(
            "fire_aspect",
            "Fire Aspect",
            2,
            Box::new(|_edition| Some(CostMultiplier { item: 2, book: 2 })),
        ),
        SharedEnchantmentType::new(
            "fire_protection",
            "Fire Protection",
            4,
            Box::new(|_edition| Some(CostMultiplier { item: 2, book: 1 })),
        ),
        SharedEnchantmentType::new(
            "flame",
            "Flame",
            1,
            Box::new(|_edition| Some(CostMultiplier { item: 4, book: 2 })),
        ),
        SharedEnchantmentType::new(
            "fortune",
            "Fortune",
            3,
            Box::new(|_edition| Some(CostMultiplier { item: 4, book: 2 })),
        ),
        SharedEnchantmentType::new(
            "frost_walker",
            "Frost Walker",
            2,
            Box::new(|_edition| Some(CostMultiplier { item: 4, book: 2 })),
        ),
        SharedEnchantmentType::new(
            "impaling",
            "Impaling",
            5,
            Box::new(|edition| {
                match edition {
                    Edition::Java => Some(CostMultiplier { item: 4, book: 2 }),
                    Edition::Bedrock => Some(CostMultiplier { item: 1, book: 2 }),
                }
            }),
        ),
        SharedEnchantmentType::new(
            "infinity",
            "Infinity",
            1,
            Box::new(|_edition| Some(CostMultiplier { item: 8, book: 4 })),
        ),
        SharedEnchantmentType::new(
            "knockback",
            "Knockback",
            2,
            Box::new(|_edition| Some(CostMultiplier { item: 2, book: 1 })),
        ),
        SharedEnchantmentType::new(
            "looting",
            "Looting",
            3,
            Box::new(|_edition| Some(CostMultiplier { item: 4, book: 2 })),
        ),
        SharedEnchantmentType::new(
            "loyalty",
            "Loyalty",
            3,
            Box::new(|_edition| Some(CostMultiplier { item: 1, book: 1 })),
        ),
        SharedEnchantmentType::new(
            "luck_of_the_sea",
            "Luck of the Sea",
            3,
            Box::new(|_edition| Some(CostMultiplier { item: 4, book: 2 })),
        ),
        SharedEnchantmentType::new(
            "lure",
            "Lure",
            3,
            Box::new(|_edition| Some(CostMultiplier { item: 4, book: 2 })),
        ),
        SharedEnchantmentType::new(
            "mending",
            "Mending",
            1,
            Box::new(|_edition| Some(CostMultiplier { item: 4, book: 2 })),
        ),
        SharedEnchantmentType::new(
            "multishot",
            "Multishot",
            1,
            Box::new(|_edition| Some(CostMultiplier { item: 4, book: 2 })),
        ),
        SharedEnchantmentType::new(
            "piercing",
            "Piercing",
            4,
            Box::new(|_edition| Some(CostMultiplier { item: 1, book: 1 })),
        ),
        SharedEnchantmentType::new(
            "power",
            "Power",
            5,
            Box::new(|_edition| Some(CostMultiplier { item: 1, book: 1 })),
        ),
        SharedEnchantmentType::new(
            "projectile_protection",
            "Projectile Protection",
            4,
            Box::new(|_edition| Some(CostMultiplier { item: 2, book: 1 })),
        ),
        SharedEnchantmentType::new(
            "protection",
            "Protection",
            4,
            Box::new(|_edition| Some(CostMultiplier { item: 1, book: 1 })),
        ),
        SharedEnchantmentType::new(
            "punch",
            "Punch",
            2,
            Box::new(|_edition| Some(CostMultiplier { item: 4, book: 2 })),
        ),
        SharedEnchantmentType::new(
            "quick_charge",
            "Quick Charge",
            3,
            Box::new(|_edition| Some(CostMultiplier { item: 2, book: 1 })),
        ),
        SharedEnchantmentType::new(
            "respiration",
            "Respiration",
            3,
            Box::new(|_edition| Some(CostMultiplier { item: 4, book: 2 })),
        ),
        SharedEnchantmentType::new(
            "riptide",
            "Riptide",
            3,
            Box::new(|_edition| Some(CostMultiplier { item: 4, book: 2 })),
        ),
        SharedEnchantmentType::new(
            "sharpness",
            "Sharpness",
            5,
            Box::new(|_edition| Some(CostMultiplier { item: 1, book: 1 })),
        ),
        SharedEnchantmentType::new(
            "silk_touch",
            "Silk Touch",
            1,
            Box::new(|_edition| Some(CostMultiplier { item: 8, book: 4 })),
        ),
        SharedEnchantmentType::new(
            "smite",
            "Smite",
            5,
            Box::new(|_edition| Some(CostMultiplier { item: 2, book: 1 })),
        ),
        SharedEnchantmentType::new(
            "soul_speed",
            "Soul Speed",
            3,
            Box::new(|_edition| Some(CostMultiplier { item: 8, book: 4 })),
        ),
        SharedEnchantmentType::new(
            "sweeping_edge",
            "Sweeping Edge",
            3,
            Box::new(|edition| {
                match edition {
                    Edition::Java => Some(CostMultiplier { item: 4, book: 2 }),
                    Edition::Bedrock => None, // Not in Bedrock
                }
            }),
        ),
        SharedEnchantmentType::new(
            "swift_sneak",
            "Swift Sneak",
            3,
            Box::new(|_edition| Some(CostMultiplier { item: 8, book: 4 })),
        ),
        SharedEnchantmentType::new(
            "thorns",
            "Thorns",
            3,
            Box::new(|_edition| Some(CostMultiplier { item: 8, book: 4 })),
        ),
        SharedEnchantmentType::new(
            "unbreaking",
            "Unbreaking",
            3,
            Box::new(|_edition| Some(CostMultiplier { item: 2, book: 1 })),
        ),
        SharedEnchantmentType::new(
            "wind_burst",
            "Wind Burst",
            3,
            Box::new(|_edition| Some(CostMultiplier { item: 4, book: 2 })),
        ),
    ]
}
