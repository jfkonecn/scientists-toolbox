use super::boxed_label::*;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct UnitInputProps {
    pub label: String,
    pub id: String,
    pub unit: String,
    #[prop_or_else(Callback::noop)]
    pub onchange: Callback<Option<f64>>,
}

#[function_component(UnitInput)]
pub fn unit_input(
    UnitInputProps {
        id,
        label,
        unit,
        onchange,
    }: &UnitInputProps,
) -> Html {
    let oninput = onchange.reform(|e: InputEvent| {
        let target: Option<EventTarget> = e.target();
        let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
        let value_result = input.map(|input| input.value().parse::<f64>());
        if let Some(Ok(value)) = value_result {
            Some(value)
        } else {
            None
        }
    });
    html! {
        <BoxedLabel id={id.clone()} label={label.clone()}>
            <input
                type={"number"}
                id={id.clone()}
                placeholder={"enter a number"}
                class={classes!( "px-3", "py-2", "w-[calc(100%-theme(spacing.16))]", "h-full")}
                oninput={oninput}
                />
                <select
                    id={format!("{}-left-dropdown", id)}
                    class={classes!("w-16", "h-full", "pl-2", "bg-white")}
                >
                    <option value={unit.clone()}>{unit}</option>
                </select>
        </BoxedLabel>
    }
}
