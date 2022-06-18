use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BoxedLabProps {
    pub label: String,
    pub id: String,
    pub children: Children,
}

#[function_component(BoxedLab)]
pub fn base_input(
    BoxedLabProps {
        id,
        label,
        children,
    }: &BoxedLabProps,
) -> Html {
    html! {
        <div class={classes!("relative", "m-3", "h-12",  "w-64", "border-2", "rounded-md", "border-gray-200")}>
            <label for={id.clone()} class={classes!("absolute", "-top-3", "left-2", "bg-white", "px-2")}>{label}</label>
            {children.clone()}
        </div>
    }
}
