use super::CostMultiplier;
use bon::{Builder, builder};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, Serialize, Deserialize)]
pub struct EnchantmentKindId(Rc<str>);

impl EnchantmentKindId {
    pub fn new(value: impl Into<Rc<str>>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<T> From<T> for EnchantmentKindId
where
    T: Into<Rc<str>>,
{
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl AsRef<str> for EnchantmentKindId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<EnchantmentKindId> for EnchantmentKindId {
    fn as_ref(&self) -> &EnchantmentKindId {
        self
    }
}

#[derive(Eq, PartialEq, Clone, Hash, Debug, Serialize, Deserialize, Builder)]
pub struct EnchantmentKind {
    #[builder(into)]
    pub id: EnchantmentKindId,

    #[builder(into)]
    pub name: Rc<str>,

    pub max_level: u8,

    pub cost_multiplier: CostMultiplier,
}

impl AsRef<EnchantmentKindId> for EnchantmentKind {
    fn as_ref(&self) -> &EnchantmentKindId {
        &self.id
    }
}
