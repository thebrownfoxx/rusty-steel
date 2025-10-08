use super::{Enchant, Error, ErrorKind, Result};
use crate::combine::enchantments;
use crate::enchantment::Enchantment;
use crate::item::Item;
use std::rc::Rc;

pub struct EnchantmentsCombined<Combine>
where
    Combine: enchantments::Combine,
{
    combine_strategy: Rc<Combine>,
}

impl<Combine> EnchantmentsCombined<Combine>
where
    Combine: enchantments::Combine,
{
    pub fn with_strategy(combine_strategy: Rc<Combine>) -> Self {
        Self { combine_strategy }
    }
}

impl<Combine> Enchant for EnchantmentsCombined<Combine>
where
    Combine: enchantments::Combine,
{
    fn enchant(&self, item: &mut Item, enchantment: Enchantment) -> Result<()> {
        let matching_enchantment = item
            .enchantments
            .iter_mut()
            .find(|existing| existing.kind == enchantment.kind);

        let Some(target) = matching_enchantment else {
            return Err(Error {
                enchantment,
                kind: ErrorKind::EnchantmentsIncompatible,
            });
        };

        Ok(self.combine_strategy.combine(target, enchantment)?)
    }
}
