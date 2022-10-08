use crate::units::{RawUnit, Unit};

use super::boxed_label::*;
use super::*;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement, HtmlSelectElement};
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
        <BoxedLabel id={id.clone()} label={label.clone()} label_type={LabelType::Input}>
            <input
                type={"number"}
                min="0"
                inputmode="numeric"
                pattern="[0-9]*"
                id={id.clone()}
                placeholder={"enter a number"}
                class={classes!("inline-block", "px-3", "py-2", "w-[calc(100%-theme(spacing.24))]", "h-full")}
                oninput={oninput}
                />
                <select
                    id={format!("{}-unit", id)}
                    class={classes!("inline-block", "w-20", "h-full", "ml-4", "px-3", "py-2", "bg-white")}
                >
                    <option value={unit.clone()}>{unit}</option>
                </select>
        </BoxedLabel>
    }
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct UnitInputProps2<T: PartialEq + TryFrom<RawUnit> + Into<RawUnit>> {
    pub id: String,
    pub label: String,
    #[prop_or_else(Callback::noop)]
    pub onchange: Callback<Option<T>>,
}

#[function_component(UnitInput2)]
pub fn unit_input2<T: Unit + PartialEq + TryFrom<RawUnit> + Into<RawUnit> + 'static>(
    UnitInputProps2 {
        id,
        label,
        onchange,
    }: &UnitInputProps2<T>,
) -> Html {
    let selected_unit_ref = use_mut_ref(|| Some(T::get_si_unit_string()));
    let selected_unit = (*selected_unit_ref.borrow())
        .clone()
        .unwrap_or("".to_owned());
    let unit_value_ref = use_mut_ref(|| None);
    let oninput = onchange.reform(
        move |(maybe_value_opt, maybe_unit_opt): (Option<Option<f64>>, Option<Option<String>>)| {
            let value_opt = {
                if let Some(value_opt) = maybe_value_opt {
                    *unit_value_ref.borrow_mut() = value_opt;
                    value_opt
                } else {
                    (*unit_value_ref.borrow()).clone()
                }
            };

            let unit_opt = {
                if let Some(unit_opt) = maybe_unit_opt {
                    *selected_unit_ref.borrow_mut() = unit_opt.clone();
                    unit_opt
                } else {
                    (*selected_unit_ref.borrow()).clone()
                }
            };
            if let (Some(value), Some(unit_display)) = (value_opt, unit_opt) {
                let raw_unit = RawUnit {
                    value,
                    unit_display,
                };
                if let Ok(val) = T::try_from(raw_unit) {
                    Some(val)
                } else {
                    None
                }
            } else {
                None
            }
        },
    );
    let options = T::list_unit_strings();
    let on_value_input = {
        let oninput = oninput.clone();
        Callback::from(move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            let value_result = input.map(|input| input.value().parse::<f64>());
            let unit_value_opt = {
                if let Some(Ok(value)) = value_result {
                    Some(value)
                } else {
                    None
                }
            };
            oninput.emit((Some(unit_value_opt), None));
        })
    };
    let on_unit_change = {
        let oninput = oninput.clone();
        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());
            let unit_opt = input.map(|input| input.value());
            oninput.emit((None, Some(unit_opt)));
        })
    };
    html! {
        <BoxedLabel id={id.clone()} label={label.clone()} label_type={LabelType::Input}>
            <input
                type={"number"}
                min="0"
                inputmode="numeric"
                pattern="[0-9]*"
                id={id.clone()}
                placeholder={"enter a number"}
                class={classes!("inline-block", "px-3", "py-2", "w-[calc(100%-theme(spacing.24))]", "h-full")}
                oninput={on_value_input}
                />
                <select
                    id={format!("{}-unit", id)}
                    onchange={on_unit_change}
                    class={classes!("inline-block", "w-20", "h-full", "ml-4", "px-3", "py-2", "bg-white")}
                >
                    {
                        options.into_iter().map(|x| {
                            html! {
                                <option
                                    selected={x == selected_unit}
                                    value={x.clone()}
                                    >{x}</option>
                            }
                        }).collect::<Html>()
                    }
                </select>
        </BoxedLabel>
    }
}
