#![recursion_limit = "512"]

mod app;
mod native;
mod utils;

#[cfg(feature = "std_web")]
mod react_stdweb;
#[cfg(feature = "std_web")]
use react_stdweb as react;
#[cfg(not(feature = "std_web"))]
mod react;
#[cfg(not(feature = "std_web"))]
pub mod react_button;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is the entry point for the web app
#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    // utils::set_panic_hook();
    web_logger::init();
    // yew::start_app::<app::App>();
    Ok(())
}
