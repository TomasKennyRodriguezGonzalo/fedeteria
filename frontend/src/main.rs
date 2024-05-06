pub mod app;
pub mod Molecules;
pub mod Components;
pub mod Pages;
pub mod router;
pub mod store;
pub mod Types;
use crate::app::App;


fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
