use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <header class={classes!("bg-sky-100", "h-20")}>
                <ul>
                    <li>
                        <a href="https://github.com/jfkonecn/scientists-toolbox">{"Github"}</a>
                    </li>
                </ul>
            </header>
            <main class={classes!("bg-white", "min-h-[calc(100vh-theme(spacing.20)-theme(spacing.20))]")}>
            <p class={classes!("bg-red-100")}>{"Test!"}</p>
            </main>
            <footer class={classes!("bg-sky-100", "h-20")}>
            </footer>
       </>
    }
}
