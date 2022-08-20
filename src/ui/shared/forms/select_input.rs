use super::boxed_label::*;
use super::*;
use std::{convert::*, fmt::Display};
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlSelectElement};
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct SelectInputProps<T: PartialEq> {
    pub label: String,
    pub id: String,
    #[prop_or_else(Callback::noop)]
    pub onchange: Callback<Option<T>>,
    pub options: Vec<T>,
    pub value: Option<T>,
}

#[function_component(SelectInput)]
pub fn select_input<T: PartialEq + TryFrom<String> + Display + Clone + Into<String> + 'static>(
    SelectInputProps {
        id,
        label,
        onchange,
        options,
        value,
    }: &SelectInputProps<T>,
) -> Html {
    let select_ref = use_node_ref();
    let on_select_change = onchange.reform(|e: Event| -> Option<T> {
        let target: Option<EventTarget> = e.target();
        let input = target.and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());
        if let Some(x) = input {
            T::try_from(x.value()).ok()
        } else {
            None
        }
    });

    let mut sorted_opts = options.clone();
    sorted_opts.sort_by_key(|x| x.to_string());

    let input_val = value.clone();
    let select_ref_clone = select_ref.clone();
    use_effect(move || {
        if let Some(val) = input_val {
            if let Some(node) = select_ref_clone.cast::<HtmlSelectElement>() {
                node.set_value(val.into().as_str());
            }
        }
        || {}
    });

    html! {
        <BoxedLabel id={id.clone()} label={label.clone()} label_type={LabelType::Input}>
            <select
                id={id.clone()}
                ref={select_ref}
                class={classes!("inline-block", "w-full", "h-full", "px-3", "py-2", "bg-white")}
                onchange={on_select_change}
            >
                {
                    sorted_opts.into_iter().map(|opt| {
                        let value: String = opt.clone().into();
                        html! {
                            <option value={value}>{opt.to_string()}</option>
                        }
                    }).collect::<Html>()
                }
            </select>
        </BoxedLabel>
    }
}
