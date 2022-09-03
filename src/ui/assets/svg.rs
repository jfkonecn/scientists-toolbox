use yew::{function_component, html, Classes, Properties};

#[derive(Properties, PartialEq)]
pub struct SvgProps {
    pub class: Option<Classes>,
}

#[function_component(MagnifyingGlass)]
pub fn magnifying_glass(SvgProps { class }: &SvgProps) -> Html {
    html! {
        <div class={class}>
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                <path stroke-linecap="round" stroke-linejoin="round" d="M21 21l-5.197-5.197m0 0A7.5 7.5 0 105.196 5.196a7.5 7.5 0 0010.607 10.607z" />
            </svg>
        </div>
    }
}

#[function_component(XMark)]
pub fn x_mark(SvgProps { class }: &SvgProps) -> Html {
    html! {
        <div class={class}>
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
            </svg>
        </div>
    }
}
