use js_sys::Function;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

use crate::utils::create_element;

#[wasm_bindgen]
pub struct Button(HtmlElement);

#[wasm_bindgen]
impl Button {
    #[wasm_bindgen(constructor)]
    pub fn new(label: &str) -> Button {
        let body = create_element("button");
        body.set_text_content(label.into());
        Button(body)
    }

    /// Set the button's action handler.
    pub fn action(self, handle: &Function) -> Self {
        self.0.set_onclick(handle.into());
        self
    }

    #[wasm_bindgen(getter)]
    pub fn body(&self) -> HtmlElement {
        self.0.clone()
    }
}
