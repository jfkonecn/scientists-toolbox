pub mod root_finders;

#[derive(Debug, PartialEq)]
pub enum RootFinderErr {
    ToleranceBelowZero,
    MaxIterationsReached,
}
