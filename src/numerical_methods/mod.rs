pub mod root_finders;

#[derive(Debug, Clone, PartialEq)]
pub enum RootFinderErr {
    ToleranceBelowZero,
    MaxIterationsReached,
}
