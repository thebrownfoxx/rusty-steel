use crate::combine::enchantments::{
    Combine as CombineEnchantments, Error as CombineEnchantmentsError,
    Result as CombineEnchantmentsResult,
};
use crate::enchant::{Enchant, Error, Result};
use crate::enchantment::{Enchantment, EnchantmentKindId};
use crate::item::Item;

pub struct MatchEnchantments<Combine: CombineEnchantments> {
    pub combine_strategy: Combine,
}

impl<Combine: CombineEnchantments> Enchant for MatchEnchantments<Combine> {
    fn enchant(&self, item: &mut Item, enchantment: Enchantment) -> Result {
        let Some(match_result) = self.combine_matching(&mut item.enchantments, &enchantment) else {
            item.enchantments.push(enchantment);
            return Ok(());
        };

        match match_result {
            Match::Combined => Ok(()),
            Match::Incompatible { conflict } => Err(Error::EnchantmentsIncompatible { conflict }),
        }
    }
}

enum Match {
    Combined,
    Incompatible { conflict: EnchantmentKindId },
}

impl<Combine: CombineEnchantments> MatchEnchantments<Combine> {
    fn combine_matching(
        &self,
        enchantments: &mut [Enchantment],
        enchantment: &Enchantment,
    ) -> Option<Match> {
        enchantments
            .iter_mut()
            .find_map(|target| self.combine_match(target, enchantment))
    }

    fn combine_match(&self, target: &mut Enchantment, sacrifice: &Enchantment) -> Option<Match> {
        let combine_result = self
            .combine_strategy
            .combine(target, sacrifice);

        match_result(target, combine_result)
    }
}

fn match_result(
    target: impl AsRef<EnchantmentKindId>,
    combine_result: CombineEnchantmentsResult,
) -> Option<Match> {
    let Err(error) = combine_result else {
        return Some(Match::Combined);
    };

    match error {
        CombineEnchantmentsError::EnchantmentKindsIncompatible => None,
        CombineEnchantmentsError::LevelsIncompatible => {
            let conflict = target.as_ref().clone();
            Some(Match::Incompatible { conflict })
        }
    }
}
