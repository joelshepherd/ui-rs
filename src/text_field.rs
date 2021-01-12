use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

use crate::{stream::Stream, utils::create_element};

#[wasm_bindgen]
pub struct TextField(HtmlInputElement);

#[wasm_bindgen]
impl TextField {
    #[wasm_bindgen(constructor)]
    pub fn new(stream: &mut Stream) -> TextField {
        let body: HtmlInputElement = create_element("input").unchecked_into();

        body.set_type("text");

        let on_subscribe = {
            let body = body.clone();
            Box::new(move |x: &str| {
                body.set_value(&x);
            })
        };
        stream.subscribe(on_subscribe);

        let on_input = {
            let body = body.clone();
            let stream = stream.clone();
            Closure::wrap(Box::new(move || {
                stream.next(body.value());
            }) as Box<dyn Fn()>)
        };
        body.set_oninput(Some(on_input.as_ref().unchecked_ref()));
        on_input.forget();

        TextField(body)
    }

    /// Obfuscate the input of the text field.
    pub fn secure(self) -> Self {
        self.0.set_type("password");
        self
    }

    #[wasm_bindgen(getter)]
    pub fn body(&self) -> HtmlInputElement {
        self.0.clone()
    }
}
