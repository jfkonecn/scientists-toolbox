use crate::ui::app::{MainRoute, ThermoRoute};
use crate::ui::assets::svg::*;
use strum::IntoEnumIterator;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement, InputEvent};
use yew::{classes, function_component, html, use_state, Callback, Properties};

#[derive(Debug, Clone, PartialEq)]
pub enum AllRoutes {
    ThermoRoute(ThermoRoute),
}

#[derive(Debug, Clone, PartialEq)]
struct SearchableGroup {
    tags: Vec<String>,
    label: String,
    configs: Vec<SearchableLinkConfig>,
}

#[derive(Debug, Clone, PartialEq)]
struct SearchableLinkConfig {
    tags: Vec<String>,
    label: String,
    route: AllRoutes,
}

fn create_link_configs() -> Vec<SearchableGroup> {
    let main_routes = SearchableGroup {
        tags: vec![],
        label: "Main".to_owned(),
        configs: MainRoute::iter()
            .map(|route| match route {
                MainRoute::Home
                | MainRoute::NotFound
                | MainRoute::Thermo
                | MainRoute::ThermoRoot => None,
            })
            .flatten()
            .collect::<Vec<SearchableLinkConfig>>(),
    };
    let thermo_routes = SearchableGroup {
        tags: vec![],
        label: "Thermodynamics".to_owned(),
        configs: ThermoRoute::iter()
            .map(|route| {
                let opt = match route {
                    ThermoRoute::SteamTable => Some(("Steam Table".to_owned(), vec![])),
                    ThermoRoute::NotFound => None,
                };
                if let Some((label, tags)) = opt {
                    Some(SearchableLinkConfig {
                        tags,
                        label,
                        route: AllRoutes::ThermoRoute(route),
                    })
                } else {
                    None
                }
            })
            .flatten()
            .collect::<Vec<SearchableLinkConfig>>(),
    };
    vec![main_routes, thermo_routes]
        .iter()
        .filter(|x| x.configs.len() > 0)
        .map(|x| (*x).clone())
        .collect::<Vec<SearchableGroup>>()
}

#[derive(Properties, PartialEq)]
struct SearchResultProps {
    group: SearchableGroup,
}

#[function_component(SearchResult)]
fn search_result(SearchResultProps { group }: &SearchResultProps) -> html {
    html! {
        <output>{group.label.to_owned()}</output>
    }
}

#[function_component(Search)]
pub fn search() -> Html {
    let search_opt = use_state(|| -> Option<String> { None });
    let on_search_change = {
        let search_opt = search_opt.clone();
        Callback::from(move |val| {
            search_opt.set(val);
        })
    };
    let oninput = on_search_change.reform(|e: InputEvent| {
        let target: Option<EventTarget> = e.target();
        let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
        let value_result = input.map(|input| input.value().parse::<String>());
        if let Some(Ok(value)) = value_result {
            Some(value)
        } else {
            None
        }
    });
    let configs = create_link_configs();
    html! {
        <form class={classes!("grid", "w-full", "content-start")}>
            <fieldset class={classes!("h-10", "relative")}>
                <div
                class={classes!("grid", "h-full","absolute",
                "left-0", "place-items-center")}
                >
                <MagnifyingGlass
                    class={classes!("h-7", "w-7", "py-1", "pl-2")}
                    />
                </div>
                <input
                class={classes!("inline-block", "pl-10",
                 "pr-3", "py-2", "w-full", "h-full",
                 "rounded-full", "shadow-md",
                "border-2")}
                    type={"text"}
                    placeholder={"Search..."}
                    oninput={oninput}
                />
            </fieldset>
            <div>{
                if let Some(ref val) = *search_opt {
                    val.clone()
                } else {
                    "".to_string()
                }
            }</div>
            {
                for
                configs.iter().map(|x| {
                    html! {
                        <SearchResult group={x.clone()} />
                    }
                })
            }
        </form>
    }
}
