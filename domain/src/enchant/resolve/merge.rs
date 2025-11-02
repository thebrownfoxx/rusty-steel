use crate::builder::Builder;
use crate::enchant::resolve::{EnchantmentsResolution as Resolution, ResolveEnchantments};
use crate::enchantment::combine::{CombineEnchantments, CombineEnchantmentsError as CombineError};
use crate::enchantment::Enchantment;

#[derive(Debug)]
pub struct Merge<Combine, Fallback>
where
    Combine: CombineEnchantments,
    Fallback: ResolveEnchantments,
{
    combiner: Combine,
    fallback: Fallback,
}

impl<Combine, Fallback> ResolveEnchantments for Merge<Combine, Fallback>
where
    Combine: CombineEnchantments,
    Fallback: ResolveEnchantments,
{
    fn resolve(&self, target: &mut Enchantment, sacrifice: &Enchantment) -> Resolution {
        let Err(error) = self.combiner.combine(target, sacrifice) else {
            return Resolution::Resolved;
        };

        match error {
            CombineError::IncompatibleKinds => self.fallback.resolve(target, sacrifice),
            CombineError::IncompatibleLevels => Resolution::Incompatible,
        }
    }
}

pub trait MergeBuilder<Combine, Fallback>
where
    Combine: CombineEnchantments,
    Fallback: ResolveEnchantments,
{
    fn merge(self, combiner: Combine) -> Builder<Merge<Combine, Fallback>>;
}

impl<Combine, Fallback> MergeBuilder<Combine, Fallback> for Builder<Fallback>
where
    Combine: CombineEnchantments,
    Fallback: ResolveEnchantments,
{
    fn merge(self, combiner: Combine) -> Builder<Merge<Combine, Fallback>> {
        self.reimplement(|fallback| Merge { combiner, fallback })
    }
}
