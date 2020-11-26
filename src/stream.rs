use wasm_bindgen::prelude::*;

static mut VALUE: Vec<String> = Vec::new();
static mut SINKS: Vec<Vec<Box<dyn Fn(&str)>>> = Vec::new();

/// Create an observable stream of variables.
#[wasm_bindgen]
#[derive(Clone)]
pub struct Stream {
    index: usize,
    // TODO: cannot yet have generic type
    //       and un-unsafe this by removing the static lists
    // value: T
    // sinks: Vec<Rc<dyn Fn(&T)>>,
}

#[wasm_bindgen]
impl Stream {
    #[wasm_bindgen(constructor)]
    pub fn new(value: String) -> Stream {
        unsafe {
            VALUE.push(value);
            SINKS.push(Vec::new());

            let index = VALUE.len() - 1;

            Stream { index }
        }
    }

    pub(crate) fn subscribe(&mut self, sink: Box<dyn Fn(&str)>) {
        unsafe {
            sink(&VALUE[self.index]);
            SINKS[self.index].push(sink);
        }
    }

    /// Push a new value onto the stream.
    pub fn next(&self, value: String) {
        unsafe {
            for sink in SINKS[self.index].iter() {
                (sink)(&value);
            }
            VALUE[self.index] = value;
        }
    }

    // TODO: crashes wasm-pack for some reason
    // #[wasm_bindgen(getter)]
    // pub fn value(&self) -> String {
    //     unsafe { VALUE[self.index].to_string() }
    // }
}
