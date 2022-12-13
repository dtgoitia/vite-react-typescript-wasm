use wasm_bindgen::prelude::*;

// Import the \`window.alert\` function from the Web.
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct CounterState {
    counter: i32,
}

#[wasm_bindgen]
impl CounterState {
    pub fn new() -> CounterState {
        let counter = 0;

        CounterState {
            counter,
        }
    }

    pub fn increment_counter(&mut self) -> i32 {
        self.counter = self.counter + 1;
        let s = format!("incrementing counter to {}", self.counter);
        log(&s);
        self.get_counter()
    }

    pub fn get_counter(&self) -> i32 {
        self.counter
    }
}
