use crate::enchantment_incompatibility_matrix::get_enchantment_incompatibility_matrix;
use crate::enchantment_kinds::get_enchantment_kinds;
use crate::item_enchantment_compatibility_matrix::get_item_enchantment_compatibility_matrix;
use crate::item_kinds::get_item_kinds;
use serde::Serialize;
use std::fs::File;
use std::io::Write;
use std::path::Path;

mod enchantment_incompatibility_matrix;
mod enchantment_kinds;
mod item_enchantment_compatibility_matrix;
mod item_kinds;
mod enchantment_kind_ids;

fn main() {
    write_json_file("../assets/enchantment_kinds.json", &get_enchantment_kinds());
    write_json_file("../assets/item_kinds.json", &get_item_kinds());
    write_json_file(
        "../assets/enchantment_incompatibility_matrix.json",
        &get_enchantment_incompatibility_matrix(),
    );
    write_json_file(
        "../assets/item_enchantment_compatibility_matrix.json",
        &get_item_enchantment_compatibility_matrix(),
    );
}

fn write_json_file(path: impl AsRef<Path>, serializable: &impl Serialize) {
    let mut file = File::create(path).unwrap();
    let content =
        serde_json::to_string_pretty(&serializable).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}
