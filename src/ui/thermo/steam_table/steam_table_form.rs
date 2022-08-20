use std::fmt::Display;

use super::super::super::shared::forms::select_input::*;
use super::super::super::shared::forms::str_output::*;
use super::super::super::shared::forms::unit_input::*;
use super::super::super::shared::forms::unit_output::*;
use super::super::super::shared::forms::*;
use crate::numerical_methods::*;
use crate::thermo::steam::*;
use crate::thermo::*;
use crate::ui::js_bindings::console_log;
use crate::ui::thermo::steam_table::steam_table_form::iapws97::get_steam_table_entry;
use yew::prelude::*;

fn entry_to_html(entry_opt: &Option<Result<PtvEntry, SteamQueryErr>>) -> Html {
    match entry_opt {
        Some(Ok(entry)) => {
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
                    <StrOutput
                        id={"temperature_output"}
                        label={"Phase"}
                        value={entry.phase_region.to_string()}
                        output_type={OutputType::Success}
                    />
                    <UnitOutput
                        id={"internal_energy_output"}
                        label={"Internal Energy"}
                        unit={"J/kg"}
                        value={entry.internal_energy}
                    />
                    <UnitOutput
                        id={"enthalpy_output"}
                        label={"Enthalpy"}
                        unit={"J/kg"}
                        value={entry.enthalpy}
                    />
                    <UnitOutput
                        id={"entropy_output"}
                        label={"Entropy"}
                        unit={"J/(kg * K)"}
                        value={entry.entropy}
                    />
                    <UnitOutput
                        id={"cv_output"}
                        label={"Isochoric Heat Capacity"}
                        unit={"J/(kg * K)"}
                        value={entry.cv}
                    />
                    <UnitOutput
                        id={"cp_output"}
                        label={"Isobaric Heat Capacity"}
                        unit={"J/(kg * K)"}
                        value={entry.cp}
                    />
                    <UnitOutput
                        id={"speed_of_sound_output"}
                        label={"Speed of Sound"}
                        unit={"m/s"}
                        value={entry.speed_of_sound}
                    />
                    <UnitOutput
                        id={"specific_volume_output"}
                        label={"Specific Volume"}
                        unit={"m^3/kg"}
                        value={entry.specific_volume}
                    />
                </fieldset>
            }
        }
        Some(Err(err)) => {
            let (label, err_msg) = match err {
                SteamQueryErr::OutOfRange(range_err) => {
                    let range_err_msg = match range_err {
                        OutOfRange::AboveCriticalPressure => "Above Critical Pressure",
                        OutOfRange::AboveCriticalTemperature => "Above Critical Temperature",
                        OutOfRange::BelowCriticalTemperature => "Below Critical Temperature",
                        OutOfRange::PressureHigh => "Pressure is High",
                        OutOfRange::PressureLow => "Pressure is Low",
                        OutOfRange::TemperatureHigh => "Temperature is High",
                        OutOfRange::TemperatureLow => "Temperature is Low",
                    };
                    (
                        String::from("Out of Range Error"),
                        String::from(range_err_msg),
                    )
                }
                SteamQueryErr::CompositePhaseRegionErr(composite_err) => {
                    let composite_err_msg = match composite_err {
                        CompositePhaseRegionErr::FractionsDoNotAddUpToOne => {
                            "Fractions Do Not Add Up To One"
                        }
                        CompositePhaseRegionErr::FractionsMustBePositive => {
                            "Fractions Must Sum to 1"
                        }
                    };
                    (
                        String::from("Phase Composition Error"),
                        String::from(composite_err_msg),
                    )
                }
                SteamQueryErr::FailedToConverge(converge_err) => {
                    let converge_err_msg = match converge_err {
                        RootFinderErr::ToleranceBelowZero => "Tolerance Below Zero",
                        RootFinderErr::MaxIterationsReached => "Max Iterations Reached",
                    };
                    (
                        String::from("Converge Error"),
                        String::from(converge_err_msg),
                    )
                }
            };
            html! {
                <fieldset class={classes!("grid", "grid-cols-1", "md:grid-cols-2", "lg:grid-cols-3")}>
                    <StrOutput
                        id={"temperature_output"}
                        label={label}
                        value={err_msg}
                        output_type={OutputType::Error}
                    />
                </fieldset>
            }
        }
        None => html! {},
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum SteamQueryType {
    PtQuery,
    SatTQuery,
    SatPQuery,
    EntropyPQuery,
    EnthalpyPQuery,
}

impl TryFrom<String> for SteamQueryType {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "PtQuery" => Ok(SteamQueryType::PtQuery),
            "SatPQuery" => Ok(SteamQueryType::SatPQuery),
            "SatTQuery" => Ok(SteamQueryType::SatTQuery),
            "EntropyPQuery" => Ok(SteamQueryType::EntropyPQuery),
            "EnthalpyPQuery" => Ok(SteamQueryType::EnthalpyPQuery),
            _ => Err(format!("Unknown Query \"{}\"", value).to_owned()),
        }
    }
}

impl Into<String> for SteamQueryType {
    fn into(self) -> String {
        match self {
            SteamQueryType::PtQuery => "PtQuery".to_owned(),
            SteamQueryType::SatPQuery => "SatPQuery".to_owned(),
            SteamQueryType::SatTQuery => "SatTQuery".to_owned(),
            SteamQueryType::EntropyPQuery => "EntropyPQuery".to_owned(),
            SteamQueryType::EnthalpyPQuery => "EnthalpyPQuery".to_owned(),
        }
    }
}

impl Display for SteamQueryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SteamQueryType::PtQuery => "Pressure Temperature",
                SteamQueryType::EnthalpyPQuery => "Enthalpy and Pressure",
                SteamQueryType::EntropyPQuery => "Entropy and Pressure",
                SteamQueryType::SatTQuery => "Saturated Temperature Steam",
                SteamQueryType::SatPQuery => "Saturated Pressure Steam",
            }
        )
    }
}

#[derive(Properties, PartialEq)]
struct SteamTableInputProps {}

#[function_component(SteamTableInput)]
fn steam_table_input(SteamTableInputProps {}: &SteamTableInputProps) -> Html {
    let query_type_opt = use_state(|| -> Option<SteamQueryType> { Some(SteamQueryType::PtQuery) });

    let pressure_opt = use_state(|| -> Option<f64> { None });
    let on_pressure_change = Callback::from(move |val| {
        pressure_opt.set(val);
    });

    let temperature_opt = use_state(|| -> Option<f64> { None });
    let on_temperature_change = Callback::from(move |val| {
        temperature_opt.set(val);
    });

    let entropy_opt = use_state(|| -> Option<f64> { None });
    let on_entropy_change = Callback::from(move |val| {
        entropy_opt.set(val);
    });

    let enthalpy_opt = use_state(|| -> Option<f64> { None });
    let on_enthalpy_change = Callback::from(move |val| {
        enthalpy_opt.set(val);
    });

    let query_value = *query_type_opt.clone();
    html! {
    <>
        <SelectInput<SteamQueryType>
            id="query_type"
            label="Query Type"
            onchange={Callback::from(move |x: Option<SteamQueryType>| {
                query_type_opt.set(x.clone());
            })}
            value={query_value}
            options={vec![
                SteamQueryType::PtQuery,
                SteamQueryType::EnthalpyPQuery,
                SteamQueryType::EntropyPQuery,
                SteamQueryType::SatTQuery,
                SteamQueryType::SatPQuery,
                ]}
        />
        {
        match query_value {
            Some(SteamQueryType::PtQuery)
            | Some(SteamQueryType::SatPQuery)
            | Some(SteamQueryType::EnthalpyPQuery)
            | Some(SteamQueryType::EntropyPQuery) => {
                    html! {
        <UnitInput id={"pressure"} label={"Pressure"} unit={"Pa"} onchange={on_pressure_change} />
                    }
            },
            Some(SteamQueryType::SatTQuery)
            | None => {
                    html! {
                        <></>
                    }
            },
        }
        }
        {
        match query_value {
            Some(SteamQueryType::PtQuery)
            | Some(SteamQueryType::SatTQuery)
             => {
                    html! {
        <UnitInput id={"temperature"} label={"Temperature"} unit={"K"} onchange={on_temperature_change}/>
                    }
            },
            Some(SteamQueryType::SatPQuery)
            | Some(SteamQueryType::EnthalpyPQuery)
            | Some(SteamQueryType::EntropyPQuery)
            | None => {
                    html! {
                        <></>
                    }
            },
        }
        }
        {
        match query_value {
            Some(SteamQueryType::EnthalpyPQuery)
             => {
                    html! {
        <UnitInput id={"enthalpy"} label={"Enthalpy"} unit={"J/kg"} onchange={on_enthalpy_change}/>
                    }
            },
            Some(SteamQueryType::SatPQuery)
            | Some(SteamQueryType::SatTQuery)
            | Some(SteamQueryType::PtQuery)
            | Some(SteamQueryType::EntropyPQuery)
            | None => {
                    html! {
                        <></>
                    }
            },
        }
        }
        {
        match query_value {
            Some(SteamQueryType::EntropyPQuery)
             => {
                    html! {
        <UnitInput id={"entropy"} label={"Entropy"} unit={"J/(kg * K)"} onchange={on_entropy_change}/>
                    }
            },
            Some(SteamQueryType::SatPQuery)
            | Some(SteamQueryType::SatTQuery)
            | Some(SteamQueryType::PtQuery)
            | Some(SteamQueryType::EnthalpyPQuery)
            | None => {
                    html! {
                        <></>
                    }
            },
        }
        }
    </>
    }
}

#[derive(Properties, PartialEq)]
pub struct SteamTableFormProps {}

#[function_component(SteamTableForm)]
pub fn steam_table_form(SteamTableFormProps {}: &SteamTableFormProps) -> Html {
    let entry_opt = use_state(|| -> Option<Result<PtvEntry, SteamQueryErr>> {
        // Some(Ok(PtvEntry {
        //     temperature: 473.15,
        //     pressure: 40e6,
        //     phase_region: PhaseRegion::NonCritical(NonCriticalPhaseRegion::Liquid),
        //     internal_energy: 825.228016170348e3,
        //     enthalpy: 870.124259682489e3,
        //     entropy: 2.275752861241e3,
        //     cv: 3.292858637199e3,
        //     cp: 4.315767590903e3,
        //     speed_of_sound: 1457.418351596083,
        //     specific_volume: 0.001122406088,
        // }))
        // Some(Err(SteamQueryErr::OutOfRange(
        //     OutOfRange::AboveCriticalPressure,
        // )))
        None
    });
    let pressure_opt = use_state(|| -> Option<f64> { None });
    let temperature_opt = use_state(|| -> Option<f64> { None });

    let output: Html = entry_to_html(&*entry_opt);

    let form_values = match (*pressure_opt, *temperature_opt) {
        (Some(pressure), Some(temperature)) => Some((pressure, temperature)),
        _ => None,
    };

    let on_pressure_change = Callback::from(move |val| {
        pressure_opt.set(val);
    });

    let on_temperature_change = Callback::from(move |val| {
        temperature_opt.set(val);
    });

    html! {
        <form class={classes!(
                "w-full",
                "h-full",
                "grid",
                "place-items-center",
                "[&>*]:border-b-2",
                "[&>*]:w-full",
                "[&>*]:border-gray-400",
                "[&>*]:p-8",
                "[&>*]:grid",
                "[&>*]:place-items-center",
            )}>
            <fieldset class={classes!("grid", "grid-cols-1", "md:grid-cols-2", "lg:grid-cols-3")}>
                <SteamTableInput/>
                <UnitInput id={"pressure"} label={"Pressure"} unit={"Pa"} onchange={on_pressure_change} />
                <UnitInput id={"temperature"} label={"Temperature"} unit={"K"} onchange={on_temperature_change}/>
            </fieldset>
            <fieldset>
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
                        if let Some((pressure, temperature)) = form_values {
                            let result = get_steam_table_entry(SteamQuery::PtQuery(PtPoint {
                                pressure,
                                temperature,
                            }));
                            entry_opt.set(Some(result));
                        }

                    }}/>
            </fieldset>
            {output}
        </form>
    }
}
