#![allow(clippy::let_unit_value, unused_variables)]
use super::assets::svg::*;
use super::fluids::orifice_plate_form::OrificePlateForm;
use super::logo::*;
use super::shared::search_button::*;
use super::splash::Splash;
use super::thermo::steam_table::steam_table_form::*;
use strum_macros::EnumIter;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Copy, Routable, PartialEq, Debug, EnumIter)]
pub enum MainRoute {
    #[at("/")]
    Home,
    #[at("/Thermo")]
    ThermoRoot,
    #[at("/Thermo/*")]
    Thermo,
    #[at("/Fluids")]
    FluidsRoot,
    #[at("/Fluids/*")]
    Fluids,
    #[not_found]
    #[at("/*")]
    NotFound,
}

#[derive(Clone, Copy, Routable, PartialEq, Debug, EnumIter)]
pub enum ThermoRoute {
    #[at("/Thermo/SteamTable")]
    SteamTable,
    #[not_found]
    #[at("/*")]
    NotFound,
}

#[derive(Clone, Copy, Routable, PartialEq, Debug, EnumIter)]
pub enum FluidsRoute {
    #[at("/Fluids/OrificePlate")]
    OrificePlate,
    #[not_found]
    #[at("/*")]
    NotFound,
}

#[function_component(NotFound)]
fn not_found() -> Html {
    html! {
        <h1>{"404 Page Not Found"}</h1>
    }
}
fn switch_thermo(route: &ThermoRoute) -> Html {
    match route {
        ThermoRoute::SteamTable => html! {
            <SteamTableForm/>
        },
        ThermoRoute::NotFound => html! {
            <NotFound/>
        },
    }
}

fn switch_fluids(route: &FluidsRoute) -> Html {
    match route {
        FluidsRoute::OrificePlate => html! {
            <OrificePlateForm />
        },
        FluidsRoute::NotFound => html! {
            <NotFound/>
        },
    }
}

fn switch_main(route: &MainRoute) -> Html {
    html! {
        <AppShell>
        {
            match route {
                MainRoute::Home => html! {
                    <Splash/>
                },
                MainRoute::ThermoRoot | MainRoute::Thermo => html! {
                    <Switch<ThermoRoute> render={Switch::render(switch_thermo)} />
                },
                MainRoute::FluidsRoot | MainRoute::Fluids => html! {
                    <Switch<FluidsRoute> render={Switch::render(switch_fluids)} />
                },
                MainRoute::NotFound => html! {
                    <NotFound/>
                },
            }
        }
        </AppShell>
    }
}

#[derive(Properties, PartialEq)]
pub struct AppShellProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(AppShell)]
fn app_shell(AppShellProps { children }: &AppShellProps) -> Html {
    let show_search_on_nav_bar =
        if let Some(location) = use_location().and_then(|x| x.route::<MainRoute>()) {
            MainRoute::Home != location
        } else {
            true
        };
    html! {
        <div class={classes!("flex", "items-center", "justify-center", "flex-col", "w-full", "bg-gray-100")}>
            <header class={classes!("bg-sky-100","h-32", "w-full",
                    "grid", "justify-center")}>
                <nav class={classes!("max-w-5xl", "w-screen", "px-10",
                "flex", "items-center", "justify-center", )}>
                    <ul class={classes!("flex", "items-center", "h-full")}>
                        <li class="w-20">
                            <Link<MainRoute> classes={classes!("hover:underline")} to={MainRoute::Home}>
                                <Logo aria_label={"Homepage"}  class={"w-full stroke-red-900"}/>
                            </Link<MainRoute>>
                        </li>
                    </ul>
                    <div class={classes!("flex-grow")}></div>
                    <ul class={classes!("flex", "items-center", "gap-6", "h-full")}>
                        {
                            if show_search_on_nav_bar {
                                html! {
                                    <li class={classes!("w-10", "h-10")}>
                                        <SearchButton/>
                                    </li>
                                }
                            } else {
                                html! {
                                    <></>
                                }
                            }
                        }
                        <li class={classes!("w-10", "h-10")}>
                            <a  target="_blank" href="https://github.com/jfkonecn/scientists-toolbox">
                                <Github class={classes!("bg-sky-100", "w-full", "h-full")}/>
                            </a>
                        </li>
                    </ul>
                </nav>
            </header>
            <main class={classes!("bg-white",
            "min-h-[calc(100vh-theme(spacing.28)-theme(spacing.20))]",
            "w-full","flex", "items-start", "justify-center",
           )}>
            <div class={classes!("max-w-5xl", "w-screen")}>
                {for children.iter()}
            </div>
            </main>
            <footer class={classes!("bg-sky-100", "h-20", "w-full")}>
            </footer>
        </div>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<MainRoute> render={Switch::render(switch_main)} />
        </BrowserRouter>
    }
}
