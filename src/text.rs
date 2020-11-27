use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

use crate::{stream::Stream, utils::create_element};

#[wasm_bindgen]
pub struct Text(HtmlElement);

#[wasm_bindgen]
impl Text {
    #[wasm_bindgen(constructor)]
    pub fn new(label: Option<String>) -> Text {
        let body = create_element("span");
        body.set_text_content(label.as_deref());
        Text(body)
    }

    pub fn stream(self, stream: &mut Stream) -> Self {
        let _body = self.0.clone();
        stream.subscribe(Box::new(move |x| {
            _body.set_text_content(Some(&x));
        }));
        self
    }

    #[wasm_bindgen(getter)]
    pub fn body(&self) -> HtmlElement {
        self.0.clone()
    }
}
