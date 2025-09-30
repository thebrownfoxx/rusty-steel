use crate::combine::cost::enchantment::EnchantmentCost;
use crate::combine::cost::enchantment::zero_enchantment_cost::ZeroEnchantmentCost;
use crate::enchantment::{Enchantment, EnchantmentKindId, EnchantmentsCompatible};
use crate::item::Item;
use std::rc::Rc;

pub struct IncompatibleEnchantmentPenalty {
    pub enchantment_cost: Box<dyn EnchantmentCost>,
    pub enchantments_compatible: Rc<dyn EnchantmentsCompatible>,
    pub penalty: u8,
}

impl IncompatibleEnchantmentPenalty {
    fn new(enchantment_compatibility: Rc<dyn EnchantmentsCompatible>, penalty: u8) -> Self {
        ZeroEnchantmentCost.add_incompatible_enchantment_penalty(enchantment_compatibility, penalty)
    }

    fn are_compatible(
        &self,
        enchantment_a: &dyn AsRef<EnchantmentKindId>,
        enchantment_b: &dyn AsRef<EnchantmentKindId>,
    ) -> bool {
        self.enchantments_compatible
            .are_compatible(enchantment_a, enchantment_b)
    }
}

impl EnchantmentCost for IncompatibleEnchantmentPenalty {
    fn enchantment_cost(&self, target: &Item, enchantment: &Enchantment) -> u8 {
        target
            .enchantments
            .iter()
            .filter(|target_enchantment| !self.are_compatible(target_enchantment, enchantment))
            .count() as u8
            * self.penalty
    }
}

pub trait AddIncompatibleEnchantmentPenalty {
    fn add_incompatible_enchantment_penalty(
        self,
        enchantments_compatible: Rc<dyn EnchantmentsCompatible>,
        penalty: u8,
    ) -> IncompatibleEnchantmentPenalty;
}

impl<T> AddIncompatibleEnchantmentPenalty for T
where
    T: EnchantmentCost + 'static,
{
    fn add_incompatible_enchantment_penalty(
        self,
        enchantments_compatible: Rc<dyn EnchantmentsCompatible>,
        penalty: u8,
    ) -> IncompatibleEnchantmentPenalty {
        IncompatibleEnchantmentPenalty {
            enchantment_cost: Box::new(self),
            enchantments_compatible,
            penalty,
        }
    }
}
