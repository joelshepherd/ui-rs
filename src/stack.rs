use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

use crate::utils::{create_element, View};

#[wasm_bindgen]
pub enum Orientation {
    Horizontal,
    Vertical,
}

#[wasm_bindgen]
pub struct Stack(HtmlElement);

#[wasm_bindgen]
impl Stack {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Stack {
        Stack(create_element("div"))
    }

    /// Append a child to the stack.
    pub fn child(self, child: &View) -> Self {
        self.0.append_child(&child.body()).unwrap();
        self
    }

    /// Orient the stack horizontally or vertically.
    pub fn orient(self, orientation: Orientation) -> Self {
        let direction = match orientation {
            Orientation::Horizontal => "row",
            Orientation::Vertical => "column",
        };

        self.0.style().set_property("display", "flex").unwrap();
        self.0
            .style()
            .set_property("flex-direction", direction)
            .unwrap();

        self
    }

    #[wasm_bindgen(getter)]
    pub fn body(&self) -> HtmlElement {
        self.0.clone()
    }
}
