use crate::enchant::resolve::{EnchantmentsResolution as Resolution, ResolveEnchantments};
use crate::enchantment::combine::{CombineEnchantments, CombineEnchantmentsError as CombineError};
use crate::enchantment::Enchantment;

#[derive(Debug)]
pub struct MergeResolveEnchantments<Combine: CombineEnchantments, Fallback: ResolveEnchantments> {
    combiner: Combine,
    fallback: Fallback,
}

impl<Combine: CombineEnchantments, Fallback: ResolveEnchantments> ResolveEnchantments
    for MergeResolveEnchantments<Combine, Fallback>
{
    fn resolve(&self, target: &mut Enchantment, sacrifice: &Enchantment) -> Resolution {
        let Err(error) = self.combiner.combine(target, sacrifice) else {
            return Resolution::Handled;
        };

        match error {
            CombineError::EnchantmentKindsIncompatible => Resolution::Unhandled,
            CombineError::LevelsIncompatible => Resolution::Incompatible {
                conflict: target.kind.clone(),
            },
        }
    }
}

pub trait Merge<Combine: CombineEnchantments, Fallback: ResolveEnchantments> {
    fn merge(self, combiner: Combine) -> MergeResolveEnchantments<Combine, Fallback>;
}

impl<Combine: CombineEnchantments, Fallback: ResolveEnchantments> Merge<Combine, Fallback>
    for Fallback
{
    fn merge(self, combiner: Combine) -> MergeResolveEnchantments<Combine, Fallback> {
        MergeResolveEnchantments {
            combiner,
            fallback: self,
        }
    }
}
