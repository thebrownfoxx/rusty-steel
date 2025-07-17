pub enum EnchantmentTypeId {
    AquaAffinity,
    BaneOfArthropods,
    BlastProtection,
    Breach,
    Channeling,
    CurseOfBinding,
    CurseOfVanishing,
    Density,
    DepthStrider,
    Efficiency,
    FeatherFalling,
    FireAspect,
    FireProtection,
    Flame,
    Fortune,
    FrostWalker,

}

pub struct EnchantmentType {
    pub id: EnchantmentTypeId,
    pub friendly_name: String,
    pub max_level: i8,
    pub item_cost_multiplier: i8,
    pub book_cost_multiplier: i8,
}