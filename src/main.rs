use crate::edition::Edition;
use crate::enchantment::enchantment_types::EnchantmentTypes;
use crate::enchantment::hardcoded::hardcoded_enchantment_types::hardcoded_enchantment_types;
use crate::enchantment::shared::shared_enchantment_types::SharedEnchantmentTypes;

mod edition;
mod enchantment;
mod item;

fn main() {
    let enchantment_types = hardcoded_enchantment_types().for_edition(Edition::Java);

    enchantment_types
        .all()
        .iter()
        .filter_map(|id| enchantment_types.get(id))
        .for_each(|enchantment_type| println!("{:?}", enchantment_type));
}
