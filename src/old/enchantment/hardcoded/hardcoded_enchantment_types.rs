use crate::enchantment::cost_multiplier::CostMultiplier;
use crate::enchantment::enchantment_type::EnchantmentTypeId;
use crate::enchantment::shared::shared_enchantment_type::SharedEnchantmentType;
use crate::enchantment::shared::shared_enchantment_types::SharedEnchantmentTypes;

pub fn hardcoded_enchantment_types() -> impl SharedEnchantmentTypes {
    vec![
        SharedEnchantmentType {
            id: EnchantmentTypeId("aqua_affinity".into()),
            name: "Aqua Affinity".into(),
            max_level: 1,
            cost_multiplier: (|_edition| CostMultiplier { item: 4, book: 2 }.into()) as fn(_) -> _,
        },
        SharedEnchantmentType {
            id: EnchantmentTypeId("bane_of_arthropods".into()),
            name: "Bane of Arthropods".into(),
            max_level: 5,
            cost_multiplier: |_edition| CostMultiplier { item: 2, book: 1 }.into(),
        },
        SharedEnchantmentType {
            id: EnchantmentTypeId("blast_protection".into()),
            name: "Blast Protection".into(),
            max_level: 4,
            cost_multiplier: |_edition| CostMultiplier { item: 4, book: 1 }.into(),
        },
        SharedEnchantmentType {
            id: EnchantmentTypeId("breach".into()),
            name: "Breach".into(),
            max_level: 4,
            cost_multiplier: |_edition| CostMultiplier { item: 4, book: 1 }.into(),
        },
    ]
}
