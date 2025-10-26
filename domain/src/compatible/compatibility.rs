use crate::compatible::AreCompatible;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
pub struct CompatibilityMap<A, B>(HashMap<A, Vec<B>>)
where
    A: Eq + Hash,
    B: PartialEq;

impl<A, B> AreCompatible<A, B> for CompatibilityMap<A, B>
where
    A: Eq + Hash,
    B: PartialEq,
{
    fn are_compatible(&self, a: &impl AsRef<A>, b: &impl AsRef<B>) -> bool {
        let Some(compatible) = self.0.get(a.as_ref()) else {
            return false;
        };

        compatible.contains(b.as_ref())
    }
}

impl<A, B> From<HashMap<A, Vec<B>>> for CompatibilityMap<A, B>
where
    A: Eq + Hash,
    B: PartialEq,
{
    fn from(value: HashMap<A, Vec<B>>) -> Self {
        Self(value)
    }
}
