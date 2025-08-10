pub mod cached_edition_variants;
pub mod edition_shared;

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub enum Edition {
    Bedrock,
    Java,
}

pub trait BorrowByEdition<T> {
    fn borrow_by_edition(&self, edition: Edition) -> &T;
}

pub trait CloneByEdition<T> {
    fn clone_by_edition(&self, edition: Edition) -> T;
}
