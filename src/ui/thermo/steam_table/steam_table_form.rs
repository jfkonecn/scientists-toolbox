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
use crate::units::Ft;
use crate::units::Length;
use crate::units::M;
use web_sys::console;
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
enum UiSteamQuery {
    PtQuery,
    SatTQuery,
    SatPQuery,
    EntropyPQuery,
    EnthalpyPQuery,
}

impl TryFrom<String> for UiSteamQuery {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "PtQuery" => Ok(UiSteamQuery::PtQuery),
            "SatPQuery" => Ok(UiSteamQuery::SatPQuery),
            "SatTQuery" => Ok(UiSteamQuery::SatTQuery),
            "EntropyPQuery" => Ok(UiSteamQuery::EntropyPQuery),
            "EnthalpyPQuery" => Ok(UiSteamQuery::EnthalpyPQuery),
            _ => Err(format!("Unknown Query \"{}\"", value).to_owned()),
        }
    }
}

impl Into<String> for UiSteamQuery {
    fn into(self) -> String {
        match self {
            UiSteamQuery::PtQuery => "PtQuery".to_owned(),
            UiSteamQuery::SatPQuery => "SatPQuery".to_owned(),
            UiSteamQuery::SatTQuery => "SatTQuery".to_owned(),
            UiSteamQuery::EntropyPQuery => "EntropyPQuery".to_owned(),
            UiSteamQuery::EnthalpyPQuery => "EnthalpyPQuery".to_owned(),
        }
    }
}

impl Display for UiSteamQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                UiSteamQuery::PtQuery => "Pressure Temperature",
                UiSteamQuery::EnthalpyPQuery => "Enthalpy and Pressure",
                UiSteamQuery::EntropyPQuery => "Entropy and Pressure",
                UiSteamQuery::SatTQuery => "Saturated Temperature Steam",
                UiSteamQuery::SatPQuery => "Saturated Pressure Steam",
            }
        )
    }
}

#[derive(Properties, PartialEq)]
struct SteamTableInputProps {
    #[prop_or_else(Callback::noop)]
    pub onchange: Callback<Option<SteamQuery>>,
}

#[function_component(SteamTableInput)]
fn steam_table_input(SteamTableInputProps { onchange }: &SteamTableInputProps) -> Html {
    let query_type_opt = use_state(|| -> Option<UiSteamQuery> { Some(UiSteamQuery::PtQuery) });
    let on_query_type_change = {
        let query_type_opt = query_type_opt.clone();
        Callback::from(move |val| {
            query_type_opt.set(val);
        })
    };

    let pressure_opt = use_state(|| -> Option<f64> { None });
    let on_pressure_change = {
        let pressure_opt = pressure_opt.clone();
        Callback::from(move |val| {
            pressure_opt.set(val);
        })
    };

    let temperature_opt = use_state(|| -> Option<f64> { None });
    let on_temperature_change = {
        let temperature_opt = temperature_opt.clone();
        Callback::from(move |val| {
            temperature_opt.set(val);
        })
    };

    let entropy_opt = use_state(|| -> Option<f64> { None });
    let on_entropy_change = {
        let entropy_opt = entropy_opt.clone();
        Callback::from(move |val| {
            entropy_opt.set(val);
        })
    };

    let enthalpy_opt = use_state(|| -> Option<f64> { None });
    let on_enthalpy_change = {
        let enthalpy_opt = enthalpy_opt.clone();
        Callback::from(move |val| {
            enthalpy_opt.set(val);
        })
    };

    let phase_region_opt = use_state(|| -> Option<SteamNonCriticalPhaseRegion> {
        Some(SteamNonCriticalPhaseRegion::Liquid)
    });
    let on_phase_region_change = {
        let phase_region_opt = phase_region_opt.clone();
        Callback::from(move |val| {
            phase_region_opt.set(val);
        })
    };

    {
        let query_type_opt = *query_type_opt.clone();
        let pressure_opt = *pressure_opt.clone();
        let temperature_opt = *temperature_opt.clone();
        let entropy_opt = *entropy_opt.clone();
        let enthalpy_opt = *enthalpy_opt.clone();
        let phase_region_opt = *phase_region_opt.clone();
        let onchange = onchange.clone();
        use_effect(move || {
            let query_opt = match query_type_opt {
                Some(UiSteamQuery::PtQuery) => match (pressure_opt, temperature_opt) {
                    (Some(p), Some(t)) => Some(SteamQuery::PtQuery(PtPoint {
                        pressure: p,
                        temperature: t,
                    })),
                    _ => None,
                },
                Some(UiSteamQuery::SatPQuery) => match (phase_region_opt, pressure_opt) {
                    (Some(r), Some(p)) => Some(SteamQuery::SatQuery(SatQuery::SatPQuery {
                        pressure: p,
                        phase_region: r,
                    })),
                    _ => None,
                },
                Some(UiSteamQuery::SatTQuery) => match (phase_region_opt, temperature_opt) {
                    (Some(r), Some(t)) => Some(SteamQuery::SatQuery(SatQuery::SatTQuery {
                        temperature: t,
                        phase_region: r,
                    })),
                    _ => None,
                },
                Some(UiSteamQuery::EnthalpyPQuery) => match (enthalpy_opt, pressure_opt) {
                    (Some(e), Some(p)) => Some(SteamQuery::EnthalpyPQuery {
                        enthalpy: e,
                        pressure: p,
                    }),
                    _ => None,
                },
                Some(UiSteamQuery::EntropyPQuery) => match (entropy_opt, pressure_opt) {
                    (Some(e), Some(p)) => Some(SteamQuery::EntropyPQuery {
                        entropy: e,
                        pressure: p,
                    }),
                    _ => None,
                },
                None => None,
            };
            onchange.emit(query_opt);
            || {}
        });
    }

    html! {
    <>
        <SelectInput<UiSteamQuery>
            id="query_type"
            label="Query Type"
            onchange={on_query_type_change}
            value={*query_type_opt}
            options={vec![
                UiSteamQuery::PtQuery,
                UiSteamQuery::EnthalpyPQuery,
                UiSteamQuery::EntropyPQuery,
                UiSteamQuery::SatTQuery,
                UiSteamQuery::SatPQuery,
                ]}
        />
        {
        match *query_type_opt {
            Some(UiSteamQuery::PtQuery)
            | Some(UiSteamQuery::SatPQuery)
            | Some(UiSteamQuery::EnthalpyPQuery)
            | Some(UiSteamQuery::EntropyPQuery) => {
                    html! {
        <UnitInput id={"pressure"} label={"Pressure"} unit={"Pa"} onchange={on_pressure_change} />
                    }
            },
            Some(UiSteamQuery::SatTQuery)
            | None => {
                    html! {
                        <></>
                    }
            },
        }
        }
        {
        match *query_type_opt {
            Some(UiSteamQuery::PtQuery)
            | Some(UiSteamQuery::SatTQuery)
             => {
                    html! {
        <UnitInput id={"temperature"} label={"Temperature"} unit={"K"} onchange={on_temperature_change}/>
                    }
            },
            Some(UiSteamQuery::SatPQuery)
            | Some(UiSteamQuery::EnthalpyPQuery)
            | Some(UiSteamQuery::EntropyPQuery)
            | None => {
                    html! {
                        <></>
                    }
            },
        }
        }
        {
        match *query_type_opt {
            Some(UiSteamQuery::EnthalpyPQuery)
             => {
                    html! {
        <UnitInput id={"enthalpy"} label={"Enthalpy"} unit={"J/kg"} onchange={on_enthalpy_change}/>
                    }
            },
            Some(UiSteamQuery::SatPQuery)
            | Some(UiSteamQuery::SatTQuery)
            | Some(UiSteamQuery::PtQuery)
            | Some(UiSteamQuery::EntropyPQuery)
            | None => {
                    html! {
                        <></>
                    }
            },
        }
        }
        {
        match *query_type_opt {
            Some(UiSteamQuery::EntropyPQuery)
             => {
                    html! {
        <UnitInput id={"entropy"} label={"Entropy"} unit={"J/(kg * K)"} onchange={on_entropy_change}/>
                    }
            },
            Some(UiSteamQuery::SatPQuery)
            | Some(UiSteamQuery::SatTQuery)
            | Some(UiSteamQuery::PtQuery)
            | Some(UiSteamQuery::EnthalpyPQuery)
            | None => {
                    html! {
                        <></>
                    }
            },
        }
        }
        {
        match *query_type_opt {
            Some(UiSteamQuery::SatPQuery)
            | Some(UiSteamQuery::SatTQuery)
             => {
                    html! {
        <SelectInput<SteamNonCriticalPhaseRegion>
            id="phase_region"
            label="Phase Region"
            onchange={on_phase_region_change}
            value={*phase_region_opt}
            options={vec![
                    SteamNonCriticalPhaseRegion::Liquid,
                    SteamNonCriticalPhaseRegion::Vapor,
                ]}
        />
                    }
            },
            Some(UiSteamQuery::EntropyPQuery)
            | Some(UiSteamQuery::PtQuery)
            | Some(UiSteamQuery::EnthalpyPQuery)
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
    let steam_query_opt = use_state_eq(|| -> Option<SteamQuery> { None });
    let on_steam_query_change = {
        let steam_query_opt = steam_query_opt.clone();
        Callback::from(move |val| {
            steam_query_opt.set(val);
        })
    };
    let entry_opt = use_state(|| -> Option<Result<PtvEntry, SteamQueryErr>> { None });

    let output: Html = entry_to_html(&*entry_opt);

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
                <SteamTableInput onchange={on_steam_query_change}/>
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
                        if let Some(query) = *steam_query_opt {
                            let result = get_steam_table_entry(query);
                            entry_opt.set(Some(result));
                        }

                    }}/>
            </fieldset>
            {output}
        </form>
    }
}
