use crate::serializer::edition_shared::SerializableEditionShared;
use crate::serializer::enchantment_kind::{map_enchantments, SerializableEnchantmentKindId};
use domain::edition::edition_shared::EditionShared;
use domain::enchantment::enchantment_kind::EnchantmentKindId;
use domain::enchantment::shared_enchantment_incompatibility_matrix::SharedEnchantmentIncompatibilityMatrix;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SerializableEnchantmentIncompatibilityMatrix(Vec<Entry>);

impl From<SerializableEnchantmentIncompatibilityMatrix> for SharedEnchantmentIncompatibilityMatrix {
    fn from(value: SerializableEnchantmentIncompatibilityMatrix) -> Self {
        SharedEnchantmentIncompatibilityMatrix(value.0.into_iter().map(map_entry).collect())
    }
}

fn map_entry(entry: Entry) -> (EnchantmentKindId, EditionShared<Vec<EnchantmentKindId>>) {
    (
        entry.enchantment.into(),
        entry.incompatible_enchantments.map(map_enchantments).into(),
    )
}

#[derive(Serialize, Deserialize, Debug)]
struct Entry {
    enchantment: SerializableEnchantmentKindId,
    incompatible_enchantments: SerializableEditionShared<Vec<SerializableEnchantmentKindId>>,
}
