use super::boxed_label::*;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct StrOutputProps {
    pub label: String,
    pub id: String,
    pub value: String,
}

#[function_component(StrOutput)]
pub fn str_input(StrOutputProps { id, label, value }: &StrOutputProps) -> Html {
    html! {
        <BoxedLabel id={id.clone()} label={label.clone()} label_type={LabelType::Output}>
            <output
                id={id.clone()}
                class={classes!("inline-block", "px-3", "py-2", "w-full", "h-full")}
            >
                {value}
            </output>
        </BoxedLabel>
    }
}
