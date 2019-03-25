mod utils;

use cfg_if::cfg_if;
use std::fmt;
use wasm_bindgen::prelude::*;

cfg_if::cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, react-wasm-rust-library!");
}

#[wasm_bindgen]
pub fn hello(name: &str) -> String {
    format!("Hello, {}", name)
}

#[wasm_bindgen]
pub fn add(num1: u32, num2: u32) -> u32 {
    num1 + num2
}

#[wasm_bindgen]
pub fn sub(num1: i32, num2: i32) -> i32 {
    num1 - num2
}
