#![allow(clippy::redundant_closure)]
pub mod app;
pub mod molecules;
pub mod components;
pub mod pages;
pub mod router;
pub mod store;
pub mod information_store;
pub mod types;
use crate::app::App;


fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
