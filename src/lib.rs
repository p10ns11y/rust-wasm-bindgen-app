extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/dom-utils.js")]
extern {
    fn alert(s: &str);
    fn appendStringToBody(s: &str);
}

#[wasm_bindgen]
pub extern fn run() {
    appendStringToBody("This is Peram from Sweden! 🏡");
    alert("This is Peram from Sweden! 🏡");
}
