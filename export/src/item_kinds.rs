use domain::edition::edition_shared::EditionShared;
use domain::item::item_kind::ItemKindId;
use domain::item::shared_item_kind::SharedItemKind;

pub const ENCHANTED_BOOK_ID: ItemKindId = ItemKindId(0);
pub const HELMET_ID: ItemKindId = ItemKindId(1);
pub const CHESTPLATE_ID: ItemKindId = ItemKindId(2);
pub const LEGGINGS_ID: ItemKindId = ItemKindId(3);
pub const BOOTS_ID: ItemKindId = ItemKindId(4);
pub const ELYTRA_ID: ItemKindId = ItemKindId(5);
pub const HEAD_ID: ItemKindId = ItemKindId(6);
pub const SWORD_ID: ItemKindId = ItemKindId(7);
pub const MACE_ID: ItemKindId = ItemKindId(8);
pub const AXE_ID: ItemKindId = ItemKindId(9);
pub const PICKAXE_ID: ItemKindId = ItemKindId(10);
pub const SHOVEL_ID: ItemKindId = ItemKindId(11);
pub const HOE_ID: ItemKindId = ItemKindId(12);
pub const BOW_ID: ItemKindId = ItemKindId(13);
pub const FISHING_ROD_ID: ItemKindId = ItemKindId(14);
pub const TRIDENT_ID: ItemKindId = ItemKindId(15);
pub const CROSSBOW_ID: ItemKindId = ItemKindId(16);
pub const SHEARS_ID: ItemKindId = ItemKindId(17);
pub const FLINT_AND_STEEL_ID: ItemKindId = ItemKindId(18);
pub const CARROT_ON_A_STICK_ID: ItemKindId = ItemKindId(19);
pub const WARPED_FUNGUS_ON_A_STICK_ID: ItemKindId = ItemKindId(20);

pub fn get_item_kinds() -> Vec<SharedItemKind> {
    vec![
        SharedItemKind::book(
            ENCHANTED_BOOK_ID,
            EditionShared::Same("Enchanted Book".into()),
        ),
        SharedItemKind::item(HELMET_ID, EditionShared::Same("Helmet".into())),
        SharedItemKind::item(CHESTPLATE_ID, EditionShared::Same("Chestplate".into())),
        SharedItemKind::item(LEGGINGS_ID, EditionShared::Same("Leggings".into())),
        SharedItemKind::item(BOOTS_ID, EditionShared::Same("Boots".into())),
        SharedItemKind::item(ELYTRA_ID, EditionShared::Same("Elytra".into())),
        SharedItemKind::item(HEAD_ID, EditionShared::Same("Head".into())),
        SharedItemKind::item(SWORD_ID, EditionShared::Same("Sword".into())),
        SharedItemKind::item(MACE_ID, EditionShared::Same("Mace".into())),
        SharedItemKind::item(AXE_ID, EditionShared::Same("Axe".into())),
        SharedItemKind::item(PICKAXE_ID, EditionShared::Same("Pickaxe".into())),
        SharedItemKind::item(SHOVEL_ID, EditionShared::Same("Shovel".into())),
        SharedItemKind::item(HOE_ID, EditionShared::Same("Hoe".into())),
        SharedItemKind::item(BOW_ID, EditionShared::Same("Bow".into())),
        SharedItemKind::item(FISHING_ROD_ID, EditionShared::Same("Fishing Rod".into())),
        SharedItemKind::item(TRIDENT_ID, EditionShared::Same("Trident".into())),
        SharedItemKind::item(CROSSBOW_ID, EditionShared::Same("Crossbow".into())),
        SharedItemKind::item(SHEARS_ID, EditionShared::Same("Shears".into())),
        SharedItemKind::item(
            FLINT_AND_STEEL_ID,
            EditionShared::Same("Flint and Steel".into()),
        ),
        SharedItemKind::item(
            CARROT_ON_A_STICK_ID,
            EditionShared::Same("Carrot on a Stick".into()),
        ),
        SharedItemKind::item(
            WARPED_FUNGUS_ON_A_STICK_ID,
            EditionShared::Same("Warped Fungus on a Stick".into()),
        ),
    ]
}
