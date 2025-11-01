mod compatibility;
mod incompatibility;

pub use compatibility::CompatibilityMap;
pub use incompatibility::IncompatibilityMap;
use std::ops::Deref;

pub trait AreCompatible<A, B = A> {
    fn are_compatible(&self, a: &impl AsRef<A>, b: &impl AsRef<B>) -> bool;
}

impl<A, B, Wrapper> AreCompatible<A, B> for Wrapper
where
    Wrapper: Deref<Target: AreCompatible<A, B>>,
{
    fn are_compatible(&self, a: &impl AsRef<A>, b: &impl AsRef<B>) -> bool {
        self.deref().are_compatible(a, b)
    }
}
