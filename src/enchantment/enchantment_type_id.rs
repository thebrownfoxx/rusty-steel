#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub struct EnchantmentTypeId(pub String);

impl From<String> for EnchantmentTypeId {
    fn from(value: String) -> Self {
        EnchantmentTypeId(value)
    }
}