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

#[function_component(NotFound)]
fn not_found() -> Html {
    html! {
        <h1>{"404 Page Not Found"}</h1>
    }
}
fn switch_thermo(route: &ThermoRoute) -> Html {
    match route {
        ThermoRoute::SteamTable => html! {
            <SteamTableForm />
        },
        ThermoRoute::NotFound => html! {
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
        <div class={classes!("flex", "items-center", "justify-center")}>
            <div class={classes!("w-full", "xl:w-[theme(screens.xl)]")}>
                <header class={classes!("bg-sky-100","flex", "items-center", "justify-center", "h-20", "p-10")}>
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
                                    <li class={classes!("w-5", "h-5")}>
                                        <SearchButton/>
                                    </li>
                                }
                            } else {
                                html! {
                                    <></>
                                }
                            }
                        }
                        <li>
                            <a class={classes!("hover:underline")} target="_blank" href="https://github.com/jfkonecn/scientists-toolbox">{"Github"}</a>
                        </li>
                    </ul>
                </header>
                <main class={classes!("bg-white", "min-h-[calc(100vh-theme(spacing.20)-theme(spacing.20))]")}>
                    {for children.iter()}
                </main>
                <footer class={classes!("bg-sky-100", "h-20")}>
                </footer>
            </div>
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
