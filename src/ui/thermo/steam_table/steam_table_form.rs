use super::super::super::shared::forms::unit_input::*;
use super::super::super::shared::forms::unit_output::*;
use crate::thermo::steam::iapws97::*;
use crate::thermo::types::*;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SteamTableFormProps {}

#[function_component(SteamTableForm)]
pub fn steam_table_form(SteamTableFormProps {}: &SteamTableFormProps) -> Html {
    let entry_opt = use_state(|| -> Option<Result<PtvEntry, SteamQueryErr>> { None });

    let on_pressure_change = Callback::from(move |val| {
        log::info!("pressure is {:?} Pa", val);
    });

    let on_temperature_change = Callback::from(move |val| {
        log::info!("temperature is  {:?} K", val);
    });

    let output: Html = if let Some(Ok(entry)) = &*entry_opt {
        html! {
            <fieldset class={classes!("grid", "grid-cols-1", "md:grid-cols-2", "lg:grid-cols-3")}>
                <UnitOutput
                    id={"pressure_output"}
                    label={"Pressure"}
                    unit={"Pa"}
                    value={entry.pressure}
                />
                <UnitOutput
                    id={"temperature_output"}
                    label={"Temperature"}
                    unit={"K"}
                    value={entry.temperature}
                />
            </fieldset>
        }
    } else {
        html! {}
    };

    html! {
        <form class={classes!("w-full", "h-full", "grid", "place-items-center")}>
            <fieldset>
            </fieldset>
            <fieldset class={classes!("grid", "grid-cols-1", "md:grid-cols-2", "lg:grid-cols-3")}>
                <UnitInput id={"pressure"} label={"Pressure"} unit={"Pa"} onchange={on_pressure_change} />
                <UnitInput id={"temperature"} label={"Temperature"} unit={"K"} onchange={on_temperature_change}/>
            </fieldset>
            <input
                value={"Calculate"}
                type="submit"
                class={classes!(
                    "hover:cursor-pointer",
                    "border-2",
                    "rounded-md",
                    "border-gray-200",
                    "p-2",
                    "w-64"
                )}
                onclick={move |e: MouseEvent| {
                    e.prevent_default();
                    let result = get_steam_table_entry(SteamQuery::PtQuery(PtPoint {
                        pressure: 10e6,
                        temperature: 300f64,
                    }));
                   entry_opt.set(Some(result));
                }}/>
                {output}
        </form>
    }
}
