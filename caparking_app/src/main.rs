use crate::components::MainComponent;

mod components;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Starting this garbage web app ! \\ö/");
    yew::start_app::<MainComponent>();
    log::info!("It has decided to exit.");
}
