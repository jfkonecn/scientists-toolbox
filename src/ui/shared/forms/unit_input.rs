use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct UnitInputProps {}

#[function_component(UnitInput)]
pub fn unit_input(UnitInputProps {}: &UnitInputProps) -> Html {
    html! {
        <input value={"test"} class={classes!("bg-red-100")}/>
    }
}
