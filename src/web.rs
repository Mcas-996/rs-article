use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn wasm_main() {
    console_error_panic_hook_set();
    web_sys::console::log_1(&"rs-grammar loaded in browser".into());
}

fn console_error_panic_hook_set() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
