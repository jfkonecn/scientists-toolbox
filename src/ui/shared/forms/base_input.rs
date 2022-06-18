use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BaseInputProps {
    pub label: String,
    pub id: String,
}

#[function_component(BaseInput)]
pub fn base_input(BaseInputProps { id, label }: &BaseInputProps) -> Html {
    html! {
        <div class={classes!("relative", "m-3")}>
            <label for={id.clone()} class={classes!("absolute", "-top-3", "left-2", "bg-white", "px-2")}>{label}</label>
            <input value={"test"} id={id.clone()} class={classes!("border-2", "rounded-md", "border-gray-200", "px-3", "py-2", "w-64")}/>
        </div>
    }
}
