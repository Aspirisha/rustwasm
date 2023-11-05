mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    let mut res: String = "Hello, ".to_owned();
    res.push_str(name);
    res.push_str(", from wasm-game-of-life!");
    alert(&res);
}
