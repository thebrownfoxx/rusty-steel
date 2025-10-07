use super::{Combine, Result};
use crate::enchantment::Enchantment;

pub struct RetainTarget;

impl Combine for RetainTarget {
    fn combine(&self, _: &mut Enchantment, _: Enchantment) -> Result {
        Ok(())
    }
}