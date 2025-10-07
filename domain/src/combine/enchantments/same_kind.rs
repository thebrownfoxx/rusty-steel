use crate::combine::enchantments::Combine;
use crate::combine::enchantments::{Error, ErrorKind, Result};
use crate::enchantment::Enchantment;

pub struct SameKind<Impl>(Impl)
where
    Impl: Combine;

impl<Impl> Combine for SameKind<Impl>
where
    Impl: Combine,
{
    fn combine(&self, target: &mut Enchantment, sacrifice: Enchantment) -> Result {
        if target.kind.clone() != sacrifice.kind.clone() {
            return Err(Error {
                sacrifice,
                kind: ErrorKind::IncompatibleEnchantments,
            });
        }

        self.0.combine(target, sacrifice)
    }
}

pub trait RequireSameKind<Impl>
where
    Impl: Combine,
{
    fn require_same_kind(self) -> SameKind<Impl>;
}

impl<Impl> RequireSameKind<Impl> for Impl
where 
    Impl: Combine,
{
    fn require_same_kind(self) -> SameKind<Impl> {
        SameKind(self)
    }
}
