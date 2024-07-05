#[allow(unused_imports)]
use app::*;

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    leptos::leptos_dom::HydrationCtx::stop_hydrating();
}
