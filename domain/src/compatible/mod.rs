mod compatibility;
mod incompatibility;

pub use compatibility::CompatibilityMap;
pub use incompatibility::IncompatibilityMap;

pub trait AreCompatible<A, B = A> {
    fn are_compatible(&self, a: &impl AsRef<A>, b: &impl AsRef<B>) -> bool;
}
