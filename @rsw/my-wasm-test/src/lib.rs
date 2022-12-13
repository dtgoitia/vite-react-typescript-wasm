use wasm_bindgen::prelude::*;

// Import the \`window.alert\` function from the Web.
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// Export a \`greet\` function from Rust to JavaScript, that alerts a
// hello message.
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("[rsw] Hello buddy, I'm {}!", name));
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
        self.get_counter()
    }

    pub fn get_counter(&self) -> i32 {
        self.counter
    }
}
