use bon::Builder;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, Serialize, Deserialize)]
pub struct ItemKindId(Rc<str>);

impl ItemKindId {
    pub fn new(value: impl Into<Rc<str>>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<T> From<T> for ItemKindId
where
    T: Into<Rc<str>>,
{
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl AsRef<str> for ItemKindId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<ItemKindId> for ItemKindId {
    fn as_ref(&self) -> &ItemKindId {
        self
    }
}

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, Builder)]
pub struct ItemKind {
    #[builder(into)]
    pub id: ItemKindId,
    #[builder(into)]
    pub name: Rc<str>,
    #[builder(default)]
    pub is_book: bool,
}

impl AsRef<ItemKindId> for ItemKind {
    fn as_ref(&self) -> &ItemKindId {
        &self.id
    }
}
