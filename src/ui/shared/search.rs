use crate::thermo::steam::SatQuery;
use crate::ui::app::{MainRoute, ThermoRoute};
use crate::ui::assets::svg::*;
use strum::IntoEnumIterator;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement, InputEvent};
use yew::{classes, function_component, html, use_state, Callback, Properties};
use yew_router::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum AllRoutes {
    MainRoute(MainRoute),
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
            .map(|route| {
                let opt: Option<(String, Vec<String>)> = match route {
                    // ThermoRoute::SteamTable => Some(("Steam Table".to_owned(), vec![])),
                    MainRoute::Home => Some(("Home".to_owned(), vec!["test".to_owned()])),
                    MainRoute::NotFound => Some((
                        "Not Found".to_owned(),
                        vec!["something".to_owned(), "test".to_owned()],
                    )),
                    MainRoute::Thermo | MainRoute::ThermoRoot => None,
                    // MainRoute::Home
                    // | MainRoute::NotFound
                    // | MainRoute::Thermo
                    // | MainRoute::ThermoRoot => None,
                };
                if let Some((label, tags)) = opt {
                    Some(SearchableLinkConfig {
                        tags,
                        label,
                        route: AllRoutes::MainRoute(route),
                    })
                } else {
                    None
                }
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
struct SearchResultLinkProps<T: Routable> {
    to: T,
    label: String,
}

#[function_component(SearchResultLink)]
fn search_link_result<T: Routable + 'static>(
    SearchResultLinkProps { to, label }: &SearchResultLinkProps<T>,
) -> html {
    let to = to.clone();
    html! {
    <Link<T>
        classes={classes!(
            "w-full", "h-full", "block",
            "justify-center", "content-center",
            "flex", "flex-col"
        )}
        to={to.clone()}
        >
        <div class={classes!("p-3")}>
            {label}
        </div>
    </Link<T>>
    }
}

#[derive(Properties, PartialEq)]
struct SearchResultProps {
    group: SearchableGroup,
}

#[function_component(SearchResult)]
fn search_result(SearchResultProps { group }: &SearchResultProps) -> html {
    html! {
        <output class={classes!("flex", "flex-col", "gap-4")}>
            <h2
            class={classes!("font-bold")}
            >
                {group.label.to_owned()}
            </h2>
            <ul class={classes!("flex", "flex-col", "gap-1")}>
            {
                for group.configs.iter().map(|config| {
                    let link = match config.route {
                        AllRoutes::MainRoute(route) => {
                            html! {
                            <SearchResultLink<MainRoute>
                                to={route}
                                label={config.label.to_owned()}
                                />

                            }
                        }
                        AllRoutes::ThermoRoute(route) => {
                            html! {
                            <SearchResultLink<ThermoRoute>
                                to={route}
                                label={config.label.to_owned()}
                                />

                            }
                        }
                    };
                    html! {

                        <li class={classes!(
                            "w-full", "h-16",
                            "rounded-lg",
                            "bg-gray-200", "hover:bg-sky-100",
                        )}>
                            {link}
                        </li>
                    }
                })
            }
            </ul>
        </output>
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
    let configs = {
        let search_opt = (*search_opt).clone().and_then(|x| {
            if x.len() > 0 {
                Some(x.to_lowercase())
            } else {
                None
            }
        });
        let configs = create_link_configs();
        if let Some(keyword) = search_opt {
            configs
                .iter()
                .map(|group| {
                    if group.label.to_lowercase().contains(&keyword)
                        || group
                            .tags
                            .iter()
                            .any(|x| x.to_lowercase().contains(&keyword))
                    {
                        Some(group.clone())
                    } else {
                        let sub_configs = group
                            .configs
                            .iter()
                            .filter(|config| {
                                config.label.to_lowercase().contains(&keyword)
                                    || config
                                        .tags
                                        .iter()
                                        .any(|x| x.to_lowercase().contains(&keyword))
                            })
                            .map(|x| x.clone())
                            .collect::<Vec<SearchableLinkConfig>>();
                        if sub_configs.len() > 0 {
                            Some(SearchableGroup {
                                label: group.label.clone(),
                                configs: sub_configs,
                                tags: group.tags.clone(),
                            })
                        } else {
                            None
                        }
                    }
                })
                .flatten()
                .collect::<Vec<SearchableGroup>>()
        } else {
            configs
        }
    };
    html! {
        <div class={classes!("flex", "flex-col", "gap-10")}>
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
                 "border-2", "rounded-md", "border-gray-200")}
                    type={"text"}
                    placeholder={"Search..."}
                    oninput={oninput}
                />
            </fieldset>
            <fieldset class={classes!("flex", "flex-col", "gap-10")}>
            {
                for configs.iter().map(|x| {
                    html! {
                        <SearchResult group={x.clone()} />
                    }
                })
            }
            </fieldset>
        </div>
    }
}
