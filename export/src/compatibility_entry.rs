macro_rules! entry {
    [$lhs:expr => $( $rhs:expr ),* $(,)?] => {
        ($lhs.clone(), EditionShared::Same(vec![$($rhs.clone(),)*]))
    };
    [$lhs:expr] => {
        entry!($lhs => )
    }
}

pub(crate) use entry;