use super::boxed_label::*;
use super::*;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct NumberOutputProps {
    pub label: String,
    pub id: String,
    pub value: f64,
}

#[function_component(NumberOutput)]
pub fn number_input(NumberOutputProps { id, label, value }: &NumberOutputProps) -> Html {
    let value_str = format!("{:.3}", value.clone());

    html! {
        <BoxedLabel id={id.clone()} label={label.clone()} label_type={LabelType::Output(OutputType::Success)}>
            <output
                id={id.clone()}
            >
                {value_str}
            </output>
        </BoxedLabel>
    }
}
