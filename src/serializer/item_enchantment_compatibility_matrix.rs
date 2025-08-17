use crate::serializer::edition_shared::SerializableEditionShared;
use crate::serializer::enchantment_kind::{map_enchantments, SerializableEnchantmentKindId};
use crate::serializer::item_kind::SerializableItemKindId;
use domain::edition::edition_shared::EditionShared;
use domain::enchantment::enchantment_kind::EnchantmentKindId;
use domain::item::item_kind::ItemKindId;
use domain::item::shared_item_enchantment_compatibility_matrix::SharedItemEnchantmentCompatibilityMatrix;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SerializableItemEnchantmentCompatibilityMatrix(Vec<Entry>);

impl From<SerializableItemEnchantmentCompatibilityMatrix>
    for SharedItemEnchantmentCompatibilityMatrix
{
    fn from(value: SerializableItemEnchantmentCompatibilityMatrix) -> Self {
        SharedItemEnchantmentCompatibilityMatrix(value.0.into_iter().map(map_entry).collect())
    }
}

fn map_entry(entry: Entry) -> (ItemKindId, EditionShared<Vec<EnchantmentKindId>>) {
    (
        entry.item.into(),
        entry.compatible_enchantments.map(map_enchantments).into(),
    )
}

#[derive(Serialize, Deserialize, Debug)]
struct Entry {
    item: SerializableItemKindId,
    compatible_enchantments: SerializableEditionShared<Vec<SerializableEnchantmentKindId>>,
}
