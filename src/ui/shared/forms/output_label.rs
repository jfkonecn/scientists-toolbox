use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct OutputLabelProps {
    pub label: String,
    pub id: String,
    pub children: Children,
}

#[function_component(OutputLabel)]
pub fn output_label(
    OutputLabelProps {
        id,
        label,
        children,
    }: &OutputLabelProps,
) -> Html {
    html! {
        <div class={classes!("relative", "m-3", "h-12",  "w-64")}>
            <label for={id.clone()} class={classes!("absolute", "-top-3", "left-2", "bg-white", "px-2")}>{label}</label>
            {children.clone()}
        </div>
    }
}
