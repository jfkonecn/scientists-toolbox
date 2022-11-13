use wasm_bindgen::prelude::*;

mod fluids;
mod numerical_methods;
mod thermo;
mod ui;
mod units;

#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<ui::app::App>();
}
