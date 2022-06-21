use super::boxed_label::*;
use super::*;
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
        <BoxedLabel id={id.clone()} label={label.clone()} label_type={LabelType::Output(OutputType::Success)}>
            <output
                id={id.clone()}
                class={classes!("inline-block", "px-3", "py-2", "w-[calc(100%-theme(spacing.24))]", "h-full")}
            >
                {format!("{:.3}", value)}
            </output>
            <output
                id={format!("{}-unit", id)}
                class={classes!("inline-block", "w-24", "h-full", "px-3", "py-2")}
            >
                {unit}
            </output>
        </BoxedLabel>
    }
}
