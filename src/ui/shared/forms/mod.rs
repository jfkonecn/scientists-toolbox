mod boxed_label;
pub mod select_input;
pub mod str_output;
pub mod unit_input;
pub mod unit_output;

#[derive(PartialEq)]
pub enum LabelType {
    Input,
    Output(OutputType),
}

#[derive(Clone, PartialEq)]
pub enum OutputType {
    Success,
    Error,
}
