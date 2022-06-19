use super::output_label::*;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct UnitOutputProps {
    pub label: String,
    pub id: String,
    pub value: f64,
    pub unit: String,
}

#[function_component(UnitOutput)]
pub fn unit_input(
    UnitOutputProps {
        id,
        label,
        value,
        unit,
    }: &UnitOutputProps,
) -> Html {
    html! {
        <OutputLabel id={id.clone()} label={label.clone()}>
            <output
                id={id.clone()}
                class={classes!( "px-3", "py-2", "w-[calc(100%-theme(spacing.16))]", "h-full")}
            >
                {value.to_string()}
            </output>
            <span
                id={format!("{}-left-dropdown", id)}
                class={classes!("w-16", "h-full", "pl-2", "bg-white")}
            >
                {unit}
            </span>
        </OutputLabel>
    }
}
