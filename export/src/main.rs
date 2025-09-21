use crate::enchantment_incompatibility_matrix::get_enchantment_incompatibility_matrix;
use crate::enchantment_kind_ids::EnchantmentKindIds;
use crate::enchantment_kinds::get_enchantment_kinds;
use crate::item_enchantment_compatibility_matrix::get_item_enchantment_compatibility_matrix;
use crate::item_kind_ids::ItemKindIds;
use crate::item_kinds::get_item_kinds;
use serde::Serialize;
use std::fs::File;
use std::io::Write;
use std::path::Path;

mod compatibility_entry;
mod enchantment_incompatibility_matrix;
mod enchantment_kind_ids;
mod enchantment_kinds;
mod item_enchantment_compatibility_matrix;
mod item_kind_ids;
mod item_kinds;

fn main() {
    let enchantment_ids = EnchantmentKindIds::default();
    let item_ids = ItemKindIds::default();

    write_json_file(
        "../assets/enchantment_kinds.json",
        &get_enchantment_kinds(&enchantment_ids),
    );
    write_json_file("../assets/item_kinds.json", &get_item_kinds(&item_ids));

    write_json_file(
        "../assets/enchantment_incompatibility_matrix.json",
        &get_enchantment_incompatibility_matrix(&enchantment_ids),
    );
    write_json_file(
        "../assets/item_enchantment_compatibility_matrix.json",
        &get_item_enchantment_compatibility_matrix(&item_ids, &enchantment_ids),
    );
}

fn write_json_file(path: impl AsRef<Path>, serializable: &impl Serialize) {
    let mut file = File::create(path).unwrap();
    let content = serde_json::to_string_pretty(&serializable).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}
