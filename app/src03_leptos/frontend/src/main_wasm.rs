use leptos::wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_app() {
    use leptos::*;
    use src03_frontend::App;

    console_error_panic_hook::set_once();

    mount_to_body(|cx| view! { cx, <App/> });
}

// When the wasm module loads, run the app
#[wasm_bindgen(start)]
pub fn main() {
    run_app();
}
