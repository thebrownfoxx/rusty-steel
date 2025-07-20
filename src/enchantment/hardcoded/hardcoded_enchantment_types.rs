use crate::edition::Edition;
use crate::enchantment::enchantment_type::{CostMultiplier, EnchantmentTypeId};
use crate::enchantment::shared::shared_enchantment_type::SharedEnchantmentType;
use crate::enchantment::shared::shared_enchantment_types::SharedEnchantmentTypes;

fn hardcoded_enchantment_types() -> impl SharedEnchantmentTypes {
    vec![SharedEnchantmentType {
        id: EnchantmentTypeId("aqua_affinity".into()),
        name: "Aqua Affinity".into(),
        cost_multiplier: |edition| {
            match edition {
                Edition::Java => CostMultiplier { item: 1, book: 1 },
                Edition::Bedrock => CostMultiplier { item: 1, book: 1 },
            }
            .into()
        },
    }]
}
