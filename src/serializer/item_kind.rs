use crate::serializer::edition_shared::SerializableEditionShared;
use domain::item::item_kind::ItemKindId;
use domain::item::shared_item_kind::SharedItemKind;
use serde::{Deserialize, Serialize};

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
}

impl From<SerializableItemKind> for SharedItemKind {
    fn from(value: SerializableItemKind) -> Self {
        SharedItemKind::new(value.id, value.name)
    }
}

impl From<SharedItemKind> for SerializableItemKind {
    fn from(value: SharedItemKind) -> Self {
        SerializableItemKind {
            id: value.id.into(),
            name: value.name.into(),
        }
    }
}
