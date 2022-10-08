use crate::units::{RawUnit, Unit};

use super::boxed_label::*;
use super::*;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlSelectElement};
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

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct UnitOutputProps2<T: PartialEq + TryFrom<RawUnit> + Into<RawUnit> + Copy> {
    pub label: String,
    pub id: String,
    pub value: T,
}

#[function_component(UnitOutput2)]
pub fn unit_input2<T: Unit + PartialEq + TryFrom<RawUnit> + Into<RawUnit> + Copy + 'static>(
    UnitOutputProps2 { id, label, value }: &UnitOutputProps2<T>,
) -> Html {
    let selected_unit = use_state(|| Some(T::get_si_unit_label().abbreviation));
    let value_str = {
        let selected_unit = {
            let select_input = selected_unit.clone();
            if let Some(ref value) = *select_input {
                value.clone()
            } else {
                "".to_owned()
            }
        };
        let value_result = value.try_convert(selected_unit);
        if let Ok(ref value) = value_result {
            let raw_unit: RawUnit = (*value).into();
            format!("{:.3}", raw_unit.value.clone())
        } else {
            "".to_owned()
        }
    };

    let on_unit_change = {
        let selected_unit = selected_unit.clone();
        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());
            let unit_opt = input.map(|input| input.value());
            selected_unit.set(unit_opt);
        })
    };
    let options = T::list_unit_labels();
    html! {
        <BoxedLabel id={id.clone()} label={label.clone()} label_type={LabelType::Output(OutputType::Success)}>
            <output
                id={id.clone()}
                class={classes!("inline-block", "px-3", "py-2", "w-[calc(100%-theme(spacing.24))]", "h-full")}
            >
                {value_str}
            </output>
            <select
                id={format!("{}-unit", id)}
                onchange={on_unit_change}
                class={classes!("inline-block", "w-20", "h-full", "ml-4", "px-3", "py-2", "bg-sky-100")}
            >
                {
                    options.into_iter().map(|x| {
                        html! {
                            <option
                                selected={Some(x.clone().abbreviation) == *selected_unit}
                                value={x.clone().abbreviation}
                                ariallabel={x.clone().plural}
                                >{x.abbreviation}</option>
                        }
                    }).collect::<Html>()
                }
            </select>
        </BoxedLabel>
    }
}
