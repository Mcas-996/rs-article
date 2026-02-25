use console_error_panic_hook;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn wasm_main() {
    console_error_panic_hook::set_once();
    web_sys::console::log_1(&"rs-grammar loaded in browser".into());
}
