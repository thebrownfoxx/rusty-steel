use crate::combine::enchantments::Combine as CombineEnchantments;
use crate::combine::enchantments::Error as CombineEnchantmentsError;
use crate::combine::enchantments::Result as CombineEnchantmentsResult;
use crate::enchant::{Enchant, Error, Result};
use crate::enchantment::{Enchantment, EnchantmentKindId};
use crate::item::Item;

pub struct Standard<Combine: CombineEnchantments> {
    pub enchantments_combine_strategy: Combine,
}

impl<Combine: CombineEnchantments> Enchant for Standard<Combine> {
    fn enchant(&self, item: &mut Item, enchantment: Enchantment) -> Result<()> {
        match self.try_combine(&mut item.enchantments, &enchantment) {
            TryCombineResult::Combined => Ok(()),
            TryCombineResult::Incompatible { conflict } => {
                Err(Error::EnchantmentsIncompatible { conflict })
            }
            TryCombineResult::NoMatch => {
                item.enchantments.push(enchantment);
                Ok(())
            }
        }
    }
}

enum TryCombineResult {
    Combined,
    Incompatible { conflict: EnchantmentKindId },
    NoMatch,
}

impl<Combine: CombineEnchantments> Standard<Combine> {
    fn try_combine(
        &self,
        enchantments: &mut Vec<Enchantment>,
        enchantment: &Enchantment,
    ) -> TryCombineResult {
        for target in enchantments.iter_mut() {
            let result = self
                .enchantments_combine_strategy
                .combine(target, enchantment);

            if let Some(result) = combine_result(target, result) {
                return result;
            }
        }

        TryCombineResult::NoMatch
    }
}

fn combine_result(
    enchantment_kind: &impl AsRef<EnchantmentKindId>,
    result: CombineEnchantmentsResult,
) -> Option<TryCombineResult> {
    if let Err(error) = result {
        return combine_error(enchantment_kind, error);
    }

    Some(TryCombineResult::Combined)
}

fn combine_error(
    enchantment_kind: &impl AsRef<EnchantmentKindId>,
    error: CombineEnchantmentsError,
) -> Option<TryCombineResult> {
    match error {
        CombineEnchantmentsError::EnchantmentKindsIncompatible => None,
        CombineEnchantmentsError::LevelsIncompatible => {
            let conflict = enchantment_kind.as_ref().clone();
            Some(TryCombineResult::Incompatible { conflict })
        }
    }
}
