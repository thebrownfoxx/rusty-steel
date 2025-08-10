use crate::edition::{BorrowByEdition, CloneByEdition, Edition};

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub enum EditionShared<T> {
    Same(T),
    Different { for_bedrock: T, for_java: T },
}

impl<T> BorrowByEdition<T> for EditionShared<T> {
    fn borrow_by_edition(&self, edition: Edition) -> &T {
        match self {
            EditionShared::Same(values) => values,
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