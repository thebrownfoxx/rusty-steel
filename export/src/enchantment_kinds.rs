use crate::enchantment_kind_ids::EnchantmentKindIds;
use domain::edition::EditionShared;
use domain::enchantment::{CostMultiplier, SharedEnchantmentKind};
use std::rc::Rc;

pub fn get_enchantment_kinds(ids: &EnchantmentKindIds) -> Vec<SharedEnchantmentKind> {
    vec![
        SharedEnchantmentKind::builder()
            .id(ids.aqua_affinity.clone())
            .name(Rc::from("Aqua Affinity"))
            .max_level(1)
            .cost_multiplier(CostMultiplier {
                for_book: 2,
                for_item: 4,
            })
            .build(),
        SharedEnchantmentKind::builder()
            .id(ids.bane_of_arthropods.clone())
            .name(Rc::from("Bane of Arthropods"))
            .max_level(5)
            .cost_multiplier(CostMultiplier {
                for_book: 1,
                for_item: 2,
            })
            .build(),
        SharedEnchantmentKind::builder()
            .id(ids.blast_protection.clone())
            .name(Rc::from("Blast Protection"))
            .max_level(4)
            .cost_multiplier(CostMultiplier {
                for_book: 2,
                for_item: 4,
            })
            .build(),
        SharedEnchantmentKind::builder()
            .id(ids.breach.clone())
            .name(Rc::from("Breach"))
            .max_level(4)
            .cost_multiplier(CostMultiplier {
                for_book: 2,
                for_item: 4,
            })
            .build(),
        SharedEnchantmentKind::builder()
            .id(ids.channeling.clone())
            .name(Rc::from("Channeling"))
            .max_level(1)
            .cost_multiplier(CostMultiplier {
                for_book: 4,
                for_item: 8,
            })
            .build(),
        SharedEnchantmentKind::builder()
            .id(ids.curse_of_binding.clone())
            .name(Rc::from("Curse of Binding"))
            .max_level(1)
            .cost_multiplier(CostMultiplier {
                for_book: 4,
                for_item: 8,
            })
            .build(),
        SharedEnchantmentKind::builder()
            .id(ids.curse_of_vanishing.clone())
            .name(Rc::from("Curse of Vanishing"))
            .max_level(1)
            .cost_multiplier(CostMultiplier {
                for_book: 4,
                for_item: 8,
            })
            .build(),
        SharedEnchantmentKind::builder()
            .id(ids.density.clone())
            .name(Rc::from("Density"))
            .max_level(5)
            .cost_multiplier(CostMultiplier {
                for_book: 1,
                for_item: 2,
            })
            .build(),
        SharedEnchantmentKind::builder()
            .id(ids.depth_strider.clone())
            .name(Rc::from("Depth Strider"))
            .max_level(3)
            .cost_multiplier(CostMultiplier {
                for_book: 2,
                for_item: 4,
            })
            .build(),
        SharedEnchantmentKind::builder()
            .id(ids.efficiency.clone())
            .name(Rc::from("Efficiency"))
            .max_level(5)
            .cost_multiplier(CostMultiplier {
                for_book: 1,
                for_item: 1,
            })
            .build(),
        SharedEnchantmentKind::builder()
            .id(ids.feather_falling.clone())
            .name(Rc::from("Feather Falling"))
            .max_level(4)
            .cost_multiplier(CostMultiplier {
                for_book: 1,
                for_item: 2,
            })
            .build(),
        SharedEnchantmentKind::builder()
            .id(ids.fire_aspect.clone())
            .name(Rc::from("Fire Aspect"))
            .max_level(2)
            .cost_multiplier(CostMultiplier {
                for_book: 2,
                for_item: 2,
            })
            .build(),
        SharedEnchantmentKind::builder()
            .id(ids.fire_protection.clone())
            .name(Rc::from("Fire Protection"))
            .max_level(4)
            .cost_multiplier(CostMultiplier {
                for_book: 1,
                for_item: 2,
            })
            .build(),
        SharedEnchantmentKind::builder()
            .id(ids.flame.clone())
            .name(Rc::from("Flame"))
            .max_level(1)
            .cost_multiplier(CostMultiplier {
                for_book: 2,
                for_item: 4,
            })
            .build(),
        SharedEnchantmentKind::builder()
            .id(ids.fortune.clone())
            .name(Rc::from("Fortune"))
            .max_level(3)
            .cost_multiplier(CostMultiplier {
                for_book: 2,
                for_item: 4,
            })
            .build(),
        SharedEnchantmentKind::builder()
            .id(ids.frost_walker.clone())
            .name(Rc::from("Frost Walker"))
            .max_level(2)
            .cost_multiplier(CostMultiplier {
                for_book: 2,
                for_item: 4,
            })
            .build(),
        SharedEnchantmentKind::builder()
            .id(ids.impaling.clone())
            .name(Rc::from("Impaling"))
            .max_level(5)
            .cost_multiplier(EditionShared::Different {
                for_bedrock: CostMultiplier {
                    for_book: 2,
                    for_item: 1,
                },
                for_java: CostMultiplier {
                    for_book: 2,
                    for_item: 4,
                },
            })
            .build(),
        SharedEnchantmentKind::builder()
            .id(ids.infinity.clone())
            .name(Rc::from("Infinity"))
            .max_level(1)
            .cost_multiplier(CostMultiplier {
                for_book: 4,
                for_item: 8,
            })
            .build(),
        SharedEnchantmentKind::builder()
            .id(ids.knockback.clone())
            .name(Rc::from("Knockback"))
            .max_level(2)
            .cost_multiplier(CostMultiplier {
                for_book: 1,
                for_item: 2,
            })
            .build(),
        SharedEnchantmentKind::builder()
            .id(ids.looting.clone())
            .name(Rc::from("Looting"))
            .max_level(3)
            .cost_multiplier(CostMultiplier {
                for_book: 2,
                for_item: 4,
            })
            .build(),
        SharedEnchantmentKind::builder()
            .id(ids.loyalty.clone())
            .name(Rc::from("Loyalty"))
            .max_level(3)
            .cost_multiplier(CostMultiplier {
                for_book: 1,
                for_item: 1,
            })
            .build(),
        SharedEnchantmentKind::builder()
            .id(ids.luck_of_the_sea.clone())
            .name(Rc::from("Luck of the Sea"))
            .max_level(3)
            .cost_multiplier(CostMultiplier {
                for_book: 2,
                for_item: 4,
            })
            .build(),
        SharedEnchantmentKind::builder()
            .id(ids.lure.clone())
            .name(Rc::from("Lure"))
            .max_level(3)
            .cost_multiplier(CostMultiplier {
                for_book: 2,
                for_item: 4,
            })
            .build(),
        SharedEnchantmentKind::builder()
            .id(ids.mending.clone())
            .name(Rc::from("Mending"))
            .max_level(1)
            .cost_multiplier(CostMultiplier {
                for_book: 2,
                for_item: 4,
            })
            .build(),
        SharedEnchantmentKind::builder()
            .id(ids.multishot.clone())
            .name(Rc::from("Multishot"))
            .max_level(1)
            .cost_multiplier(CostMultiplier {
                for_book: 2,
                for_item: 4,
            })
            .build(),
        SharedEnchantmentKind::builder()
            .id(ids.piercing.clone())
            .name(Rc::from("Piercing"))
            .max_level(4)
            .cost_multiplier(CostMultiplier {
                for_book: 1,
                for_item: 1,
            })
            .build(),
        SharedEnchantmentKind::builder()
            .id(ids.power.clone())
            .name(Rc::from("Power"))
            .max_level(5)
            .cost_multiplier(CostMultiplier {
                for_book: 1,
                for_item: 1,
            })
            .build(),
        SharedEnchantmentKind::builder()
            .id(ids.projectile_protection.clone())
            .name(Rc::from("Projectile Protection"))
            .max_level(4)
            .cost_multiplier(CostMultiplier {
                for_book: 1,
                for_item: 2,
            })
            .build(),
        SharedEnchantmentKind::builder()
            .id(ids.protection.clone())
            .name(Rc::from("Protection"))
            .max_level(4)
            .cost_multiplier(CostMultiplier {
                for_book: 1,
                for_item: 1,
            })
            .build(),
        SharedEnchantmentKind::builder()
            .id(ids.punch.clone())
            .name(Rc::from("Punch"))
            .max_level(2)
            .cost_multiplier(CostMultiplier {
                for_book: 2,
                for_item: 4,
            })
            .build(),
        SharedEnchantmentKind::builder()
            .id(ids.quick_charge.clone())
            .name(Rc::from("Quick Charge"))
            .max_level(3)
            .cost_multiplier(CostMultiplier {
                for_book: 1,
                for_item: 2,
            })
            .build(),
        SharedEnchantmentKind::builder()
            .id(ids.respiration.clone())
            .name(Rc::from("Respiration"))
            .max_level(3)
            .cost_multiplier(CostMultiplier {
                for_book: 2,
                for_item: 4,
            })
            .build(),
        SharedEnchantmentKind::builder()
            .id(ids.riptide.clone())
            .name(Rc::from("Riptide"))
            .max_level(3)
            .cost_multiplier(CostMultiplier {
                for_book: 2,
                for_item: 4,
            })
            .build(),
        SharedEnchantmentKind::builder()
            .id(ids.sharpness.clone())
            .name(Rc::from("Sharpness"))
            .max_level(5)
            .cost_multiplier(CostMultiplier {
                for_book: 1,
                for_item: 1,
            })
            .build(),
        SharedEnchantmentKind::builder()
            .id(ids.silk_touch.clone())
            .name(Rc::from("Silk Touch"))
            .max_level(1)
            .cost_multiplier(CostMultiplier {
                for_book: 4,
                for_item: 8,
            })
            .build(),
        SharedEnchantmentKind::builder()
            .id(ids.smite.clone())
            .name(Rc::from("Smite"))
            .max_level(5)
            .cost_multiplier(CostMultiplier {
                for_book: 1,
                for_item: 2,
            })
            .build(),
        SharedEnchantmentKind::builder()
            .id(ids.soul_speed.clone())
            .name(Rc::from("Soul Speed"))
            .max_level(3)
            .cost_multiplier(CostMultiplier {
                for_book: 4,
                for_item: 8,
            })
            .build(),
        SharedEnchantmentKind::builder()
            .id(ids.sweeping_edge.clone())
            .name(EditionShared::Different {
                for_java: Rc::from("Sweeping Edge"),
                for_bedrock: Rc::from(""),
            })
            .max_level(EditionShared::Different {
                for_java: 3,
                for_bedrock: 0,
            })
            .cost_multiplier(EditionShared::Different {
                for_bedrock: CostMultiplier {
                    for_book: 0,
                    for_item: 0,
                },
                for_java: CostMultiplier {
                    for_book: 2,
                    for_item: 4,
                },
            })
            .build(),
        SharedEnchantmentKind::builder()
            .id(ids.swift_sneak.clone())
            .name(Rc::from("Swift Sneak"))
            .max_level(3)
            .cost_multiplier(CostMultiplier {
                for_book: 4,
                for_item: 8,
            })
            .build(),
        SharedEnchantmentKind::builder()
            .id(ids.thorns.clone())
            .name(Rc::from("Thorns"))
            .max_level(3)
            .cost_multiplier(CostMultiplier {
                for_book: 4,
                for_item: 8,
            })
            .build(),
        SharedEnchantmentKind::builder()
            .id(ids.unbreaking.clone())
            .name(Rc::from("Unbreaking"))
            .max_level(3)
            .cost_multiplier(CostMultiplier {
                for_book: 1,
                for_item: 2,
            })
            .build(),
        SharedEnchantmentKind::builder()
            .id(ids.wind_burst.clone())
            .name(Rc::from("Wind Burst"))
            .max_level(3)
            .cost_multiplier(CostMultiplier {
                for_book: 2,
                for_item: 4,
            })
            .build(),
    ]
}
