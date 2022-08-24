use super::js_bindings::console_log;
use super::shared::search_button::*;
use super::splash::Splash;
use super::thermo::steam_table::steam_table_form::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq, Debug)]
enum MainRoute {
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

#[derive(Clone, Routable, PartialEq, Debug)]
enum ThermoRoute {
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
            <div class={classes!("w-full", "lg:w-[theme(screens.lg)]")}>
                <header class={classes!("bg-sky-100","flex", "items-center", "justify-center", "h-20", "p-10")}>
                    <ul class={classes!("flex", "items-center", "h-full")}>
                        <li>
                            <Link<MainRoute> classes={classes!("hover:underline")} to={MainRoute::Home}>
                                {"Scientist's Toolbox"}
                            </Link<MainRoute>>
                        </li>
                    </ul>
                    <div class={classes!("flex-grow")}></div>
                    <ul class={classes!("flex", "items-center", "h-full")}>
                        {
                            if show_search_on_nav_bar {
                                html! {
                                    <SearchButton button_type={SearchButtonType::Compact} />
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
