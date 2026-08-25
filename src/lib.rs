#![recursion_limit = "512"]

pub mod app;
pub mod components;
pub mod domain;
pub mod pages;
pub mod server;
pub mod tools;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
