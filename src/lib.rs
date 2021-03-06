#![recursion_limit = "512"]

mod app;
// mod login;
mod components;
mod error;

mod home;

mod index;
mod routes;

mod http;

// mod api;//will delete

mod api;

mod util;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is the entry point for the web app
#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    // yew::start_app::<app::Model>();
    // yew::start_app::<login::Login>();
    // yew::start_app::<testroute::Model>();
    // yew::start_app::<home::HomePage>();
    yew::start_app::<index::Index>();
    Ok(())
}
