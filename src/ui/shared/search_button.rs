use std::sync::Arc;

use crate::ui::app::{MainRoute, ThermoRoute};
use crate::ui::js_bindings::console_log;

use super::modal::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use web_sys::MouseEvent;
use yew::prelude::*;
use yew::{function_component, use_state_eq, Callback, Properties};
use yew_router::prelude::*;

#[derive(PartialEq)]
pub enum SearchButtonType {
    Compact,
    Wide,
}

#[derive(Debug, Clone)]
pub enum AllRoutes {
    MainRoute(MainRoute),
    ThermoRoute(ThermoRoute),
}

#[derive(Debug, Clone)]
struct SearchableGroup {
    tags: Vec<String>,
    label: String,
    configs: Vec<SearchableLinkConfig>,
}

#[derive(Debug, Clone)]
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
pub struct SearchButtonProps {
    pub button_type: SearchButtonType,
}

#[function_component(SearchButton)]
pub fn search_button(SearchButtonProps { button_type }: &SearchButtonProps) -> Html {
    let configs = create_link_configs();
    let show_modal = use_state_eq(|| false);
    let toggle_show_modal = {
        let show_modal = show_modal.clone();
        let show_modal_value = !*show_modal;
        Callback::from(move |_: MouseEvent| {
            show_modal.set(show_modal_value);
        })
    };
    let close_modal = {
        let show_modal = show_modal.clone();
        Callback::from(move |_: Event| {
            show_modal.set(false);
        })
    };

    html! {
        <>
            <button onclick={toggle_show_modal}>{"toggle modal"}</button>
            {
                if *show_modal {
                    html! {
                    <Modal
                        class={classes!("bg-red-700", "w-44", "h-52")} on_close_requested={close_modal}>
                        {
                            for
                            configs.iter().map(|x| {
                                html! {
                                    <h1>{x.label.to_owned()}</h1>
                                }
                            })
                        }
                    </Modal>

                    }
                } else {
                    html! {
                        <></>
                    }
                }
            }
        </>
    }
}
