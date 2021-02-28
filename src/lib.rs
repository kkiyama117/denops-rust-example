mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console, js_name = log)] #[wasm_bindgen(js_namespace = console, js_name = log)] fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() ->JsValue {
    // alert("Hello, denops-rust!");
    JsValue::from_str(&format!("{}","Hello, denops-rust!" ))
}
