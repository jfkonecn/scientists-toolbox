use super::boxed_label::*;
use super::*;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct NumberInputProps {
    pub id: String,
    pub label: String,
    #[prop_or_else(Callback::noop)]
    pub onchange: Callback<Option<f64>>,
}

#[function_component(NumberInput)]
pub fn number_input(
    NumberInputProps {
        id,
        label,
        onchange,
    }: &NumberInputProps,
) -> Html {
    let number_value_ref = use_mut_ref(|| None);
    let oninput = onchange.reform(move |maybe_value_opt: Option<Option<f64>>| {
        if let Some(value_opt) = maybe_value_opt {
            *number_value_ref.borrow_mut() = value_opt;
            value_opt
        } else {
            *number_value_ref.borrow()
        }
    });
    let on_number_input = {
        Callback::from(move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            let value_result = input.map(|input| input.value().parse::<f64>());
            let number_opt = {
                if let Some(Ok(value)) = value_result {
                    Some(value)
                } else {
                    None
                }
            };
            oninput.emit(Some(number_opt));
        })
    };
    html! {
        <BoxedLabel id={id.clone()} label={label.clone()} label_type={LabelType::Input}>
            <input
                type={"number"}
                min="0"
                inputmode="decimal"
                id={id.clone()}
                placeholder={"enter a number"}
                oninput={on_number_input}
                />
        </BoxedLabel>
    }
}
