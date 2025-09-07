use domain::enchantment::EnchantmentKindId;

macro_rules! enchantment_kind_ids {
    ( $( $name:ident ),+ $(,)?) => {
        pub struct EnchantmentKindIds {
            $(pub $name: EnchantmentKindId,)+
        }

        impl Default for EnchantmentKindIds {
            fn default() -> Self {
                Self {
                    $($name: stringify!($name).into(),)+
                }
            }
        }
    };
}

enchantment_kind_ids! {
    aqua_affinity,
    bane_of_arthropods,
    blast_protection,
    breach,
    channeling,
    curse_of_binding,
    curse_of_vanishing,
    density,
    depth_strider,
    efficiency,
    feather_falling,
    fire_aspect,
    fire_protection,
    flame,
    fortune,
    frost_walker,
    impaling,
    infinity,
    knockback,
    looting,
    loyalty,
    luck_of_the_sea,
    lure,
    mending,
    multishot,
    piercing,
    power,
    projectile_protection,
    protection,
    punch,
    quick_charge,
    respiration,
    riptide,
    sharpness,
    silk_touch,
    smite,
    soul_speed,
    sweeping_edge,
    swift_sneak,
    thorns,
    unbreaking,
    wind_burst,
}
