use yew::prelude::*;
use yew_router::prelude::*;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
pub mod app;
pub mod Molecules;
pub mod Components;
pub mod Pages;
use crate::app::App;


fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
