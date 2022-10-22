use yew::*;

#[derive(Properties, PartialEq)]
pub struct LogoProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub aria_label: String,
}
#[function_component(Logo)]
pub fn logo(LogoProps { class, aria_label }: &LogoProps) -> Html {
    let class = class.clone();
    let aria_label = aria_label.clone();
    html! {
            // is there a way to import the svg file instead?
    <svg aria-label={aria_label} class={class} xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
        <circle stroke="black" fill="none" cx="50" cy="50" r="35" />
        <path stroke="black" fill="none" d="M 50 05 L 89 27.5 L 89 72.5 L 50 95 L 11 72.5 L 11 27.5 Z" />
        <text fill="rgb(185 28 28)" x="50" y="50" text-anchor="middle">{"Benzene"}</text>
        <text fill="rgb(185 28 28)" x="50" y="65" text-anchor="middle">{"Box"}</text>
    </svg>
            }
}
