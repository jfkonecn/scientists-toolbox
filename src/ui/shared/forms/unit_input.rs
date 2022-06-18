use super::base_input::*;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct UnitInputProps {}

#[function_component(UnitInput)]
pub fn unit_input(UnitInputProps {}: &UnitInputProps) -> Html {
    html! {
        <BaseInput id={"test"} label={"Test"}/>
    }
}
