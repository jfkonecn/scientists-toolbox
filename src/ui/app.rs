use super::thermo::steam_table::steam_table_form::*;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class={classes!("flex", "items-center", "justify-center")}>
            <div class={classes!("w-full", "lg:w-[theme(screens.lg)]")}>
                <header class={classes!("bg-sky-100","flex", "items-center", "justify-center", "h-20", "p-10")}>
                    <ul class={classes!("flex", "items-center", "h-full")}>
                        <li>
                            <a class={classes!("hover:underline")} href="/">{"Scientist's Toolbox"}</a>
                        </li>
                    </ul>
                    <div class={classes!("flex-grow")}></div>
                    <ul class={classes!("flex", "items-center", "h-full")}>
                        <li>
                            <a class={classes!("hover:underline")} href="https://github.com/jfkonecn/scientists-toolbox">{"Github"}</a>
                        </li>
                    </ul>
                </header>
                <main class={classes!("bg-white", "min-h-[calc(100vh-theme(spacing.20)-theme(spacing.20))]")}>
                    <SteamTableForm />
                </main>
                <footer class={classes!("bg-sky-100", "h-20")}>
                </footer>
            </div>
        </div>
    }
}
