use crate::serializer::cost_multiplier::SerializableCostMultiplier;
use crate::serializer::edition_shared::SerializableEditionShared;
use domain::enchantment::enchantment_kind::EnchantmentKindId;
use domain::enchantment::shared_enchantment_kind::SharedEnchantmentKind;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SerializableEnchantmentKindId(pub String);

pub(super) fn map_enchantments(enchantments: Vec<SerializableEnchantmentKindId>) -> Vec<EnchantmentKindId> {
    enchantments
        .into_iter()
        .map(EnchantmentKindId::from)
        .collect()
}

impl From<SerializableEnchantmentKindId> for EnchantmentKindId {
    fn from(value: SerializableEnchantmentKindId) -> Self {
        EnchantmentKindId(value.0)
    }
}

impl From<EnchantmentKindId> for SerializableEnchantmentKindId {
    fn from(value: EnchantmentKindId) -> Self {
        SerializableEnchantmentKindId(value.0)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SerializableEnchantmentKind {
    pub id: SerializableEnchantmentKindId,
    pub name: SerializableEditionShared<String>,
    pub max_level: SerializableEditionShared<u8>,
    pub cost_multiplier: SerializableEditionShared<SerializableCostMultiplier>,
}

impl From<SerializableEnchantmentKind> for SharedEnchantmentKind {
    fn from(value: SerializableEnchantmentKind) -> Self {
        SharedEnchantmentKind::new(
            value.id,
            value.name,
            value.max_level,
            value.cost_multiplier.map_into(),
        )
    }
}

impl From<SharedEnchantmentKind> for SerializableEnchantmentKind {
    fn from(value: SharedEnchantmentKind) -> Self {
        SerializableEnchantmentKind {
            id: value.id.into(),
            name: value.name.into(),
            max_level: value.max_level.into(),
            cost_multiplier: value.cost_multiplier.map_into().into(),
        }
    }
}
