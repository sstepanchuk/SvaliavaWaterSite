#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    
    // Initialize console logging for `log`-based macros
    _ = console_log::init_with_level(log::Level::Debug);

    // Set up panic hooks for better debugging in the browser
    console_error_panic_hook::set_once();

    // Hydrate the app
    leptos::mount::hydrate_body(App);
}
