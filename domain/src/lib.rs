pub mod builder;
pub mod compatible;
pub mod edition;
pub mod enchant;
pub mod enchantment;
pub mod item;

enum Animal {
    Alive { hp: i32 },
    Dead,
}
