use crate::enchantment::enchantment_type::EnchantmentTypeId;
use std::collections::HashSet;

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub struct ItemTypeId(pub String);

pub struct ItemType {
    pub id: ItemTypeId,
    pub compatible_enchantment_types: HashSet<EnchantmentTypeId>,
}

pub trait ItemTypes {
    fn all(&self) -> Vec<&ItemTypeId>;
    fn get(&self, id: &ItemTypeId) -> Option<&ItemType>;
}

impl ItemTypes for Vec<ItemType> {
    fn all(&self) -> Vec<&ItemTypeId> {
        self.iter().map(|item_type| &item_type.id).collect()
    }

    fn get(&self, id: &ItemTypeId) -> Option<&ItemType> {
        self.iter().find(|item_type| item_type.id == *id)
    }
}
