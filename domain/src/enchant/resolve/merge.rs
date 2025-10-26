use crate::enchant::resolve::{EnchantmentsResolution as Resolution, ResolveEnchantments};
use crate::enchantment::combine::{CombineEnchantments, CombineEnchantmentsError as CombineError};
use crate::enchantment::Enchantment;

#[derive(Debug)]
pub struct MergeEnchantmentResolver<Combine, Fallback>
where
    Combine: CombineEnchantments,
    Fallback: ResolveEnchantments,
{
    combiner: Combine,
    fallback: Fallback,
}

impl<Combine, Fallback> ResolveEnchantments for MergeEnchantmentResolver<Combine, Fallback>
where
    Combine: CombineEnchantments,
    Fallback: ResolveEnchantments,
{
    fn resolve(&self, target: &mut Enchantment, sacrifice: &Enchantment) -> Resolution {
        let Err(error) = self.combiner.combine(target, sacrifice) else {
            return Resolution::Handled;
        };

        match error {
            CombineError::EnchantmentKindsIncompatible => self.fallback.resolve(target, sacrifice),
            CombineError::LevelsIncompatible => Resolution::Incompatible {
                conflict: target.kind.clone(),
            },
        }
    }
}

pub trait MergeResolveEnchantments<Combine, Fallback>
where
    Combine: CombineEnchantments,
    Fallback: ResolveEnchantments,
{
    fn merge(self, combiner: Combine) -> MergeEnchantmentResolver<Combine, Fallback>;
}

impl<Combine, Fallback> MergeResolveEnchantments<Combine, Fallback> for Fallback
where
    Combine: CombineEnchantments,
    Fallback: ResolveEnchantments,
{
    fn merge(self, combiner: Combine) -> MergeEnchantmentResolver<Combine, Fallback> {
        MergeEnchantmentResolver {
            combiner,
            fallback: self,
        }
    }
}
