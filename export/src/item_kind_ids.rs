use domain::item::ItemKindId;

macro_rules! item_kind_ids {
    ( $( $name:ident ),+ $(,)?) => {
        pub struct ItemKindIds {
            $(pub $name: ItemKindId,)+
        }

        impl Default for ItemKindIds {
            fn default() -> Self {
                Self {
                    $($name: stringify!($name).into(),)+
                }
            }
        }
    };
}

item_kind_ids! {
    enchanted_book,
    helmet,
    chestplate,
    leggings,
    boots,
    elytra,
    head,
    sword,
    mace,
    axe,
    pickaxe,
    shovel,
    hoe,
    bow,
    fishing_rod,
    trident,
    crossbow,
    shears,
    flint_and_steel,
    carrot_on_a_stick,
    warped_fungus_on_a_stick,
}
