use wasm_bindgen::prelude::*;

use zeblang;
mod interpret;
use interpret::interpret;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn do_compile(src: &str) {
    match zeblang::make_parsetree(src.to_string()) {
        Ok(out) => alert(&format!("{}", interpret(out).unwrap())),
        Err(_) => alert("error"),
    }
}
