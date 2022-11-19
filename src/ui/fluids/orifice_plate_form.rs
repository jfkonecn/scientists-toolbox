use crate::fluids::orifice_plate::query_orifice_plate;
use crate::fluids::OrificePlateFlow;
use crate::fluids::OrificePlateQuery;
use crate::fluids::OrificePlateQueryErr;
use crate::ui::shared::forms::calculation_button_section::*;
use crate::ui::shared::forms::calculation_form::*;
use crate::ui::shared::forms::calculation_section::*;
use crate::ui::shared::forms::number_input::NumberInput;
use crate::ui::shared::forms::number_output::*;
use crate::ui::shared::forms::select_input::*;
use crate::ui::shared::forms::str_output::*;
use crate::ui::shared::forms::unit_input::*;
use crate::ui::shared::forms::unit_output::*;
use crate::ui::shared::forms::*;
use crate::units::*;
use std::fmt::Display;
use yew::prelude::*;

#[derive(Debug, PartialEq, Copy, Clone)]
enum UiOrificePlateQuery {
    FlowRate,
}

impl TryFrom<String> for UiOrificePlateQuery {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "FlowRate" => Ok(UiOrificePlateQuery::FlowRate),
            _ => Err(format!("Unknown Query \"{}\"", value).to_owned()),
        }
    }
}

impl Into<String> for UiOrificePlateQuery {
    fn into(self) -> String {
        match self {
            UiOrificePlateQuery::FlowRate => "FlowRate".to_owned(),
        }
    }
}

impl Display for UiOrificePlateQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                UiOrificePlateQuery::FlowRate => "Flow Rate",
            }
        )
    }
}
#[derive(Properties, PartialEq)]
struct OrificePlateInputProps {
    #[prop_or_else(Callback::noop)]
    pub onchange: Callback<Option<OrificePlateQuery>>,
}

#[function_component(OrificePlateInput)]
fn orifice_plate_input(OrificePlateInputProps { onchange }: &OrificePlateInputProps) -> Html {
    let query_type_opt =
        use_state(|| -> Option<UiOrificePlateQuery> { Some(UiOrificePlateQuery::FlowRate) });
    let on_query_type_change = {
        let query_type_opt = query_type_opt.clone();
        Callback::from(move |val| {
            query_type_opt.set(val);
        })
    };

    let pipe_area_opt = use_state(|| -> Option<Area> { None });
    let on_pipe_area_change = {
        let pipe_area_opt = pipe_area_opt.clone();
        Callback::from(move |val| {
            pipe_area_opt.set(val);
        })
    };

    let orifice_area_opt = use_state(|| -> Option<Area> { None });
    let on_orifice_area_change = {
        let orifice_area_opt = orifice_area_opt.clone();
        Callback::from(move |val| {
            orifice_area_opt.set(val);
        })
    };

    let pressure_drop_opt = use_state(|| -> Option<Pressure> { None });
    let on_pressure_drop_change = {
        let pressure_drop_opt = pressure_drop_opt.clone();
        Callback::from(move |val| {
            pressure_drop_opt.set(val);
        })
    };

    let density_opt = use_state(|| -> Option<Density> { None });
    let on_density_change = {
        let density_opt = density_opt.clone();
        Callback::from(move |val| {
            density_opt.set(val);
        })
    };

    let discharge_coefficient_opt = use_state(|| -> Option<f64> { None });
    let on_discharge_coefficient_change = {
        let discharge_coefficient_opt = discharge_coefficient_opt.clone();
        Callback::from(move |val| {
            discharge_coefficient_opt.set(val);
        })
    };

    {
        let query_type_opt = *query_type_opt.clone();
        let pipe_area_opt = *pipe_area_opt.clone();
        let orifice_area_opt = *orifice_area_opt.clone();
        let pressure_drop_opt = *pressure_drop_opt.clone();
        let density_opt = *density_opt.clone();
        let discharge_coefficient_opt = *discharge_coefficient_opt.clone();
        let onchange = onchange.clone();

        use_effect(move || {
            let query_opt = match query_type_opt {
                Some(UiOrificePlateQuery::FlowRate) => match (
                    pipe_area_opt,
                    orifice_area_opt,
                    density_opt,
                    pressure_drop_opt,
                    discharge_coefficient_opt,
                ) {
                    (
                        Some(pipe_area),
                        Some(orifice_area),
                        Some(density),
                        Some(pressure_drop),
                        Some(discharge_coefficient),
                    ) => Some(OrificePlateQuery::FlowRate {
                        pipe_area,
                        orifice_area,
                        density,
                        discharge_coefficient,
                        pressure_drop,
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
        <SelectInput<UiOrificePlateQuery>
            id="query_type"
            label="Query Type"
            onchange={on_query_type_change}
            value={*query_type_opt}
            options={vec![
                UiOrificePlateQuery::FlowRate,
                ]}
        />
        {
        match *query_type_opt {
            Some(UiOrificePlateQuery::FlowRate) => {
                    html! {
        <UnitInput<Area> id={"pipe_area"} label={"Pipe Area"} onchange={on_pipe_area_change} />
                    }
            },
            | None => {
                    html! {
                        <></>
                    }
            },
        }
        }
        {
        match *query_type_opt {
            Some(UiOrificePlateQuery::FlowRate) => {
                    html! {
        <UnitInput<Area> id={"orifice_area"} label={"Orifice Area"} onchange={on_orifice_area_change} />
                    }
            },
            | None => {
                    html! {
                        <></>
                    }
            },
        }
        }
        {
        match *query_type_opt {
            Some(UiOrificePlateQuery::FlowRate) => {
                    html! {
        <UnitInput<Density> id={"density"} label={"Density"} onchange={on_density_change} />
                    }
            },
            | None => {
                    html! {
                        <></>
                    }
            },
        }
        }
        {
        match *query_type_opt {
            Some(UiOrificePlateQuery::FlowRate) => {
                    html! {
        <UnitInput<Pressure> id={"pressure_drop"} label={"Pressure Drop"} onchange={on_pressure_drop_change} />
                    }
            },
            | None => {
                    html! {
                        <></>
                    }
            },
        }
        }
        {
        match *query_type_opt {
            Some(UiOrificePlateQuery::FlowRate) => {
                    html! {
        <NumberInput id={"discharge_coefficient"} label={"Discharge Coefficient"} onchange={on_discharge_coefficient_change} />
                    }
            },
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
pub struct OrificePlateFlowOutputProps {
    flow_opt: Option<Result<OrificePlateFlow, OrificePlateQueryErr>>,
}

#[function_component(OrificePlateFlowOutput)]
fn orifice_plate_flow_output(
    OrificePlateFlowOutputProps { flow_opt }: &OrificePlateFlowOutputProps,
) -> Html {
    match flow_opt {
        Some(Ok(flow)) => {
            html! {
                <>
                    <UnitOutput<Area>
                        id={"pipe_area_output"}
                        label={"Pipe Area"}
                        value={flow.pipe_area}
                    />
                    <UnitOutput<Area>
                        id={"orifice_area_output"}
                        label={"Orifice Area"}
                        value={flow.orifice_area}
                    />
                    <UnitOutput<Density>
                        id={"density_output"}
                        label={"Density"}
                        value={flow.density}
                    />
                    <UnitOutput<Pressure>
                        id={"pressure_drop_output"}
                        label={"Pressure Drop"}
                        value={flow.pressure_drop}
                    />
                    <NumberOutput
                        id={"discharge_coefficient_output"}
                        label={"Discharge Coefficient"}
                        value={flow.discharge_coefficient}
                    />
                    <UnitOutput<VolumetricFlowRate>
                        id={"flow_rate_output"}
                        label={"Flow Rate"}
                        value={flow.flow_rate}
                    />
                </>
            }
        }
        Some(Err(err)) => {
            let (label, err_msg) = match err {
                OrificePlateQueryErr::DischargeCoefficientHigh => (
                    String::from("Discharge Coefficient Error"),
                    String::from("Discharge Coefficient is above 1"),
                ),
                OrificePlateQueryErr::DischargeCoefficientLow => (
                    String::from("Discharge Coefficient Error"),
                    String::from("Discharge Coefficient is below 0"),
                ),
            };
            html! {
                <StrOutput
                    id={"error_output"}
                    label={label}
                    value={err_msg}
                    output_type={OutputType::Error}
                />
            }
        }
        None => html! {},
    }
}

#[derive(Properties, PartialEq)]
pub struct OrificePlateFormProps {}

#[function_component(OrificePlateForm)]
pub fn steam_table_form(OrificePlateFormProps {}: &OrificePlateFormProps) -> Html {
    let orifice_query_opt = use_state_eq(|| -> Option<OrificePlateQuery> { None });
    let on_orifice_query_change = {
        let orifice_query_opt = orifice_query_opt.clone();
        Callback::from(move |val| {
            orifice_query_opt.set(val);
        })
    };
    let flow_opt = use_state(|| -> Option<Result<OrificePlateFlow, OrificePlateQueryErr>> { None });

    let flow_opt_output = (*flow_opt).clone();

    html! {
        <CalculationForm>
            <CalculationSection>
                <OrificePlateInput onchange={on_orifice_query_change}/>
            </CalculationSection>
            <CalculationButtonSection on_click={Callback::from(move |_: Event| {
                        if let Some(query) = *orifice_query_opt {
                            let result = query_orifice_plate(query);
                            flow_opt.set(Some(result));
                        }
                    })}/>
            <CalculationSection>
                <OrificePlateFlowOutput flow_opt={flow_opt_output}/>
            </CalculationSection>
        </CalculationForm>
    }
}
