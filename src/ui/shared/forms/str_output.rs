use super::boxed_label::*;
use super::*;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct StrOutputProps {
    pub label: String,
    pub id: String,
    pub value: String,
    pub output_type: OutputType,
}

#[function_component(StrOutput)]
pub fn str_input(
    StrOutputProps {
        id,
        label,
        value,
        output_type,
    }: &StrOutputProps,
) -> Html {
    html! {
        <BoxedLabel id={id.clone()} label={label.clone()} label_type={LabelType::Output(output_type.clone())}>
            <output
                id={id.clone()}
                class={classes!("inline-block", "px-3", "py-2", "w-full", "h-full")}
            >
                {value}
            </output>
        </BoxedLabel>
    }
}
