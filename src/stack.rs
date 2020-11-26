use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

use crate::utils::{create_element, ViewFromJs};

#[wasm_bindgen]
pub enum Orientation {
    Horizontal,
    Vertical,
}

#[wasm_bindgen]
pub struct Stack {
    body: HtmlElement,
}

#[wasm_bindgen]
impl Stack {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Stack {
        Stack {
            body: create_element("div"),
        }
    }

    #[wasm_bindgen(variadic)]
    pub fn child(self, child: &ViewFromJs) -> Self {
        self.body.append_child(&child.get_body()).unwrap();
        self
    }

    pub fn orient(self, orientation: Orientation) -> Self {
        let direction = match orientation {
            Orientation::Horizontal => "row",
            Orientation::Vertical => "column",
        };

        self.body.style().set_property("display", "flex").unwrap();
        self.body
            .style()
            .set_property("flex-direction", direction)
            .unwrap();

        self
    }

    #[wasm_bindgen(getter)]
    pub fn get_body(&self) -> HtmlElement {
        self.body.clone()
    }
}
