use super::{Enchant, Result};
use crate::enchantment::Enchantment;
use crate::item::Item;

#[derive(Debug)]
pub struct FallbackStrategy<Primary, Fallback>
where
    Primary: Enchant,
    Fallback: Enchant,
{
    primary: Primary,
    fallback: Fallback,
}

impl<Primary, Fallback> Enchant for FallbackStrategy<Primary, Fallback>
where
    Primary: Enchant,
    Fallback: Enchant,
{
    fn enchant(&self, item: &mut Item, enchantment: Enchantment) -> Result<()> {
        self.primary
            .enchant(item, enchantment)
            .or_else(|error| self.fallback.enchant(item, error.enchantment))
    }
}

pub trait WithFallback<Primary, Fallback>
where
    Primary: Enchant,
    Fallback: Enchant,
{
    fn with_fallback(self, fallback: Fallback) -> FallbackStrategy<Primary, Fallback>;
}

impl<Primary, Fallback> WithFallback<Primary, Fallback> for Primary
where
    Primary: Enchant,
    Fallback: Enchant,
{
    fn with_fallback(self, fallback: Fallback) -> FallbackStrategy<Primary, Fallback> {
        FallbackStrategy {
            primary: self,
            fallback,
        }
    }
}
