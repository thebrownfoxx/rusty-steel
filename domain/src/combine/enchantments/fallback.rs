use super::{Combine, Result};
use crate::enchantment::Enchantment;

pub struct FallbackStrategy<Primary, Fallback>
where
    Primary: Combine,
    Fallback: Combine,
{
    primary: Primary,
    fallback: Fallback,
}

impl<Primary, Fallback> Combine for FallbackStrategy<Primary, Fallback>
where
    Primary: Combine,
    Fallback: Combine,
{
    fn combine(&self, target: &mut Enchantment, sacrifice: Enchantment) -> Result {
        self.primary
            .combine(target, sacrifice)
            .or_else(|error| self.fallback.combine(target, error.sacrifice))
    }
}

pub trait WithFallback<Primary, Fallback>
where
    Primary: Combine,
    Fallback: Combine,
{
    fn with_fallback(self, fallback: Fallback) -> FallbackStrategy<Primary, Fallback>;
}

impl<Primary, Fallback> WithFallback<Primary, Fallback> for Primary
where
    Primary: Combine,
    Fallback: Combine,
{
    fn with_fallback(self, fallback: Fallback) -> FallbackStrategy<Primary, Fallback> {
        FallbackStrategy {
            primary: self,
            fallback,
        }
    }
}
