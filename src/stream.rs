use js_sys::Function;
use std::{cell::RefCell, collections::HashMap, sync::atomic::AtomicUsize, sync::atomic::Ordering};
use wasm_bindgen::prelude::*;

thread_local! {
    static INDEX: AtomicUsize = AtomicUsize::new(0);
    static VALUE: RefCell<HashMap<usize, String>> = RefCell::new(HashMap::new());
    static SINKS: RefCell<HashMap<usize, Vec<Box<dyn Fn(&str)>>>> = RefCell::new(HashMap::new());
}

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
        let index = INDEX.with(|x| x.fetch_add(1, Ordering::SeqCst));

        VALUE.with(|x| x.borrow_mut().insert(index, value));
        SINKS.with(|x| x.borrow_mut().insert(index, Vec::new()));

        Stream { index }
    }

    pub(crate) fn subscribe(&self, sink: Box<dyn Fn(&str)>) {
        sink(&self.value());
        SINKS.with(|x| x.borrow_mut().get_mut(&self.index).unwrap().push(sink));
    }

    #[wasm_bindgen(js_name=subscribe)]
    pub fn subscribe_for_js(&self, sink: Function) {
        self.subscribe(Box::new(move |value| {
            let null = JsValue::null();
            sink.call1(&null, &JsValue::from(value)).unwrap();
        }));
    }

    /// Push a new value onto the stream.
    pub fn next(&self, value: String) {
        SINKS.with(|x| {
            for sink in x.borrow().get(&self.index).unwrap().iter() {
                (sink)(&value);
            }
        });
        VALUE.with(|x| x.borrow_mut().insert(self.index, value));
    }

    #[wasm_bindgen(getter)]
    pub fn value(&self) -> String {
        VALUE.with(|x| x.borrow().get(&self.index).unwrap().into())
    }
}

impl Drop for Stream {
    fn drop(&mut self) {
        // TODO: Check for other references before clearing static hashmap
        // TODO: I think `Rc` or `Arc` might be the solution
        // VALUE.with(|x| x.borrow_mut().remove(&self.index));
        // SINKS.with(|x| x.borrow_mut().remove(&self.index));
    }
}
