use crate::enchantment::enchantment_type::{CostMultiplier, EnchantmentTypeId};
use crate::enchantment::shared::shared_enchantment_type::SharedEnchantmentType;
use crate::enchantment::shared::shared_enchantment_types::SharedEnchantmentTypes;

fn hardcoded_enchantment_types() -> impl SharedEnchantmentTypes {
    vec![
        SharedEnchantmentType {
            id: EnchantmentTypeId("aqua_affinity".into()),
            name: "Aqua Affinity".into(),
            cost_multiplier: (|_edition| CostMultiplier { item: 4, book: 2 }.into()) as fn(_) -> _,
        },
        SharedEnchantmentType {
            id: EnchantmentTypeId("bane_of_arthropods".into()),
            name: "Bane of Arthropods".into(),
            cost_multiplier: |_edition| CostMultiplier { item: 2, book: 1 }.into(),
        },
    ]
}
