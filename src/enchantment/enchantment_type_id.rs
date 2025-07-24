#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub struct EnchantmentTypeId(pub String);

impl From<&str> for EnchantmentTypeId {
    fn from(value: &str) -> Self {
        EnchantmentTypeId(String::from(value))
    }
}
