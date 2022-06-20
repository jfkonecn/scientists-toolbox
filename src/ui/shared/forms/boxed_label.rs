use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BoxedLabelProps {
    pub label: String,
    pub label_type: LabelType,
    pub id: String,
    pub children: Children,
}

#[derive(PartialEq)]
pub enum LabelType {
    Input,
    Output,
}

#[function_component(BoxedLabel)]
pub fn boxed_label(
    BoxedLabelProps {
        id,
        label,
        label_type,
        children,
    }: &BoxedLabelProps,
) -> Html {
    let bg_color = match label_type {
        LabelType::Input => "bg-white",
        LabelType::Output => "bg-sky-100",
    };
    html! {
        <div class={classes!("relative", "mx-3", "p-1", "mb-3", "mt-8", "h-12",  "w-72", "border-2", "rounded-md", "border-gray-200", bg_color )}>
            <label for={id.clone()} class={classes!("absolute", "-top-5", "left-2", "bg-white", "px-2", "border-2", "rounded-md", "border-gray-200",)}>{label}</label>
            {children.clone()}
        </div>
    }
}
