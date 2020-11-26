use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlElement};

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn create_element(tag: &str) -> HtmlElement {
    window()
        .unwrap()
        .document()
        .unwrap()
        .create_element(&tag)
        .unwrap()
        .dyn_ref::<HtmlElement>()
        .unwrap()
        .to_owned()
}

pub trait View {
    fn get_body(&self) -> HtmlElement;
}

#[wasm_bindgen]
extern "C" {
    pub type ViewFromJs;

    #[wasm_bindgen(structural, method, getter)]
    pub fn get_body(this: &ViewFromJs) -> HtmlElement;
}
