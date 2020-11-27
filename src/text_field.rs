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

        let _body = body.clone();
        stream.subscribe(Box::new(move |x| {
            _body.set_value(&x);
        }));

        let _body = body.clone();
        let _stream = stream.clone();
        let cb = Closure::wrap(Box::new(move || {
            _stream.next(_body.value());
        }) as Box<dyn Fn()>);
        body.set_oninput(Some(cb.as_ref().unchecked_ref()));
        cb.forget();

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
