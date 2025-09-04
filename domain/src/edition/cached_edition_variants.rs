use super::{BorrowByEdition, CloneByEdition, Edition};
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash, Debug, Serialize, Deserialize)]
pub struct CachedEditionVariants<T> {
    pub for_bedrock: T,
    pub for_java: T,
}

impl<T> CachedEditionVariants<T> {
    pub fn from(clonable: &impl CloneByEdition<T>) -> Self {
        CachedEditionVariants {
            for_bedrock: clonable.clone_by_edition(Edition::Bedrock),
            for_java: clonable.clone_by_edition(Edition::Java),
        }
    }
}

impl<T> BorrowByEdition<T> for CachedEditionVariants<T> {
    fn borrow_by_edition(&self, edition: Edition) -> &T {
        match edition {
            Edition::Bedrock => &self.for_bedrock,
            Edition::Java => &self.for_java,
        }
    }
}

pub trait CacheByEdition<T> {
    fn cache(&self) -> CachedEditionVariants<T>;
}

impl<T, Clonable: CloneByEdition<T>> CacheByEdition<T> for Clonable {
    fn cache(&self) -> CachedEditionVariants<T> {
        CachedEditionVariants::from(self)
    }
}
