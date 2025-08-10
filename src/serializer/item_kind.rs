use crate::serializer::edition_shared::SerializableEditionShared;
use crate::serializer::enchantment_kind::SerializableEnchantmentKindId;
use domain::enchantment::enchantment_kind::EnchantmentKindId;
use domain::item::item_kind::ItemKindId;
use domain::item::shared_item_kind::SharedItemKind;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug)]
pub struct SerializableItemKindId(pub String);

impl From<SerializableItemKindId> for ItemKindId {
    fn from(value: SerializableItemKindId) -> Self {
        ItemKindId(value.0)
    }
}

impl From<ItemKindId> for SerializableItemKindId {
    fn from(value: ItemKindId) -> Self {
        SerializableItemKindId(value.0)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SerializableItemKind {
    pub id: SerializableItemKindId,
    pub name: SerializableEditionShared<String>,
    pub compatible_enchantments: SerializableEditionShared<Vec<SerializableEnchantmentKindId>>,
}

impl From<SerializableItemKind> for SharedItemKind {
    fn from(value: SerializableItemKind) -> Self {
        let compatible_enchantments = value
            .compatible_enchantments
            .map(|vec| HashSet::from_iter(vec.into_iter().map(EnchantmentKindId::from)));
        SharedItemKind::new(value.id, value.name, compatible_enchantments)
    }
}

impl From<SharedItemKind> for SerializableItemKind {
    fn from(value: SharedItemKind) -> Self {
        let compatible_enchantments = value
            .compatible_enchantments
            .map(|set| Vec::from_iter(set.into_iter().map(SerializableEnchantmentKindId::from)))
            .into();
        SerializableItemKind {
            id: value.id.into(),
            name: value.name.into(),
            compatible_enchantments,
        }
    }
}
