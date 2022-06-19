use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BoxedLabelProps {
    pub label: String,
    pub id: String,
    pub children: Children,
}

#[function_component(BoxedLabel)]
pub fn boxed_label(
    BoxedLabelProps {
        id,
        label,
        children,
    }: &BoxedLabelProps,
) -> Html {
    html! {
        <div class={classes!("relative", "m-3", "h-12",  "w-64", "border-2", "rounded-md", "border-gray-200")}>
            <label for={id.clone()} class={classes!("absolute", "-top-3", "left-2", "bg-white", "px-2")}>{label}</label>
            {children.clone()}
        </div>
    }
}
