use super::js_bindings::console_log;
use super::shared::modal::*;
use super::thermo::steam_table::steam_table_form::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
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

#[derive(Clone, Routable, PartialEq)]
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
    match route {
        MainRoute::Home => html! {
            <h1>{"Home"}</h1>
        },
        MainRoute::ThermoRoot | MainRoute::Thermo => html! {
            <Switch<ThermoRoute> render={Switch::render(switch_thermo)} />
        },
        MainRoute::NotFound => html! {
            <NotFound/>
        },
    }
}

#[function_component(App)]
pub fn app() -> Html {
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
                        <li>
                            <a class={classes!("hover:underline")} target="_blank" href="https://github.com/jfkonecn/scientists-toolbox">{"Github"}</a>
                        </li>
                    </ul>
                </header>
                <main class={classes!("bg-white", "min-h-[calc(100vh-theme(spacing.20)-theme(spacing.20))]")}>
                    <button onclick={toggle_show_modal}>{"toggle modal"}</button>
                    {
                        if *show_modal {
                            html! {
                            <Modal class={classes!("bg-red-700")} on_close_requested={close_modal}>
                            <h1>{"test2"}</h1>
                            </Modal>

                            }
                        } else {
                            html! {
                                <></>
                            }
                        }
                    }
                    <BrowserRouter>
                        <Switch<MainRoute> render={Switch::render(switch_main)} />
                    </BrowserRouter>
                </main>
                <footer class={classes!("bg-sky-100", "h-20")}>
                </footer>
            </div>
        </div>
    }
}
