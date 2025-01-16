mod app;
mod components;
mod pages;
mod api;
mod router;
mod hooks;
mod config;
mod store;

use app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}