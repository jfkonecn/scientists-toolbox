use super::boxed_label::*;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct UnitInputProps {
    pub label: String,
    pub id: String,
    pub unit: String,
}

#[function_component(UnitInput)]
pub fn unit_input(UnitInputProps { id, label, unit }: &UnitInputProps) -> Html {
    html! {
        <BoxedLab id={id.clone()} label={label.clone()}>
            <input
                type={"number"}
                id={id.clone()}
                placeholder={"enter a number"}
                class={classes!( "px-3", "py-2", "w-[calc(100%-theme(spacing.16))]", "h-full")}
                />
                <select
                    id={format!("{}-left-dropdown", id)}
                    class={classes!("w-16", "h-full", "pl-2", "bg-white")}
                >
                    <option value={unit.clone()}>{unit}</option>
                </select>
        </BoxedLab>
    }
}
