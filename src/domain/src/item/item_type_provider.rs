use crate::item::item_type::{ItemType, ItemTypeId};

pub trait ItemTypeProvider {
    fn all_ids(&self) -> impl Iterator<Item = &ItemTypeId>;
    fn get(&self, id: &ItemTypeId) -> &impl ItemType;
}
