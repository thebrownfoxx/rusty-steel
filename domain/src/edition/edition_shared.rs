use super::{BorrowByEdition, CloneByEdition, Edition};
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash, Debug, Serialize, Deserialize)]
pub enum EditionShared<T> {
    Same(T),
    Different { for_bedrock: T, for_java: T },
}

impl<T> EditionShared<T> {
    pub fn map<R, F>(self, f: F) -> EditionShared<R>
    where
        F: Fn(T) -> R,
    {
        match self {
            EditionShared::Same(value) => EditionShared::Same(f(value)),
            EditionShared::Different {
                for_bedrock,
                for_java,
            } => EditionShared::Different {
                for_bedrock: f(for_bedrock),
                for_java: f(for_java),
            },
        }
    }

    pub fn map_into<R: From<T>>(self) -> EditionShared<R> {
        self.map(|t| t.into())
    }
}

impl<T> BorrowByEdition<T> for EditionShared<T> {
    fn borrow_by_edition(&self, edition: Edition) -> &T {
        match self {
            EditionShared::Same(value) => value,
            EditionShared::Different {
                for_bedrock,
                for_java,
            } => match edition {
                Edition::Bedrock => for_bedrock,
                Edition::Java => for_java,
            },
        }
    }
}

impl<T: Clone, Borrowable: BorrowByEdition<T>> CloneByEdition<T> for Borrowable {
    fn clone_by_edition(&self, edition: Edition) -> T {
        self.borrow_by_edition(edition).clone()
    }
}
