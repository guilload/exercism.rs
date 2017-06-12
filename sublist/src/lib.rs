use Comparison::*;


#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T>(short: &[T], long: &[T]) -> Comparison
    where T: std::cmp::PartialEq
{
    if short == long {
        Equal
    }
    else if short.is_empty() || long.windows(short.len()).any(|w| w == short) {
        Sublist
    }
    else if long.is_empty() || short.windows(long.len()).any(|w| w == long) {
        Superlist
    }
    else {
        Unequal
    }
}
