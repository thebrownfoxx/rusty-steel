use crate::item_kind_ids::ItemKindIds;
use domain::item::SharedItemKind;
use std::rc::Rc;

pub fn get_item_kinds(ids: &ItemKindIds) -> Vec<SharedItemKind> {
    vec![
        SharedItemKind::builder()
            .id(ids.enchanted_book.clone())
            .name(Rc::from("Enchanted Book"))
            .is_book(true)
            .build(),
        SharedItemKind::builder()
            .id(ids.helmet.clone())
            .name(Rc::from("Helmet"))
            .build(),
        SharedItemKind::builder()
            .id(ids.chestplate.clone())
            .name(Rc::from("Chestplate"))
            .build(),
        SharedItemKind::builder()
            .id(ids.leggings.clone())
            .name(Rc::from("Leggings"))
            .build(),
        SharedItemKind::builder()
            .id(ids.boots.clone())
            .name(Rc::from("Boots"))
            .build(),
        SharedItemKind::builder()
            .id(ids.elytra.clone())
            .name(Rc::from("Elytra"))
            .build(),
        SharedItemKind::builder()
            .id(ids.head.clone())
            .name(Rc::from("Head"))
            .build(),
        SharedItemKind::builder()
            .id(ids.sword.clone())
            .name(Rc::from("Sword"))
            .build(),
        SharedItemKind::builder()
            .id(ids.mace.clone())
            .name(Rc::from("Mace"))
            .build(),
        SharedItemKind::builder()
            .id(ids.axe.clone())
            .name(Rc::from("Axe"))
            .build(),
        SharedItemKind::builder()
            .id(ids.pickaxe.clone())
            .name(Rc::from("Pickaxe"))
            .build(),
        SharedItemKind::builder()
            .id(ids.shovel.clone())
            .name(Rc::from("Shovel"))
            .build(),
        SharedItemKind::builder()
            .id(ids.hoe.clone())
            .name(Rc::from("Hoe"))
            .build(),
        SharedItemKind::builder()
            .id(ids.bow.clone())
            .name(Rc::from("Bow"))
            .build(),
        SharedItemKind::builder()
            .id(ids.fishing_rod.clone())
            .name(Rc::from("Fishing Rod"))
            .build(),
        SharedItemKind::builder()
            .id(ids.trident.clone())
            .name(Rc::from("Trident"))
            .build(),
        SharedItemKind::builder()
            .id(ids.crossbow.clone())
            .name(Rc::from("Crossbow"))
            .build(),
        SharedItemKind::builder()
            .id(ids.shears.clone())
            .name(Rc::from("Shears"))
            .build(),
        SharedItemKind::builder()
            .id(ids.flint_and_steel.clone())
            .name(Rc::from("Flint and Steel"))
            .build(),
        SharedItemKind::builder()
            .id(ids.carrot_on_a_stick.clone())
            .name(Rc::from("Carrot on a Stick"))
            .build(),
        SharedItemKind::builder()
            .id(ids.warped_fungus_on_a_stick.clone())
            .name(Rc::from("Warped Fungus on a Stick"))
            .build(),
    ]
}
