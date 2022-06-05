use wasm_bindgen::prelude::*;
mod thermo;
mod ui;

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<ui::app::App>();
}
