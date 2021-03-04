// extern WASM calls are wrapped in unsafe,
// but they don't technically have to be.
#![deny(unused_unsafe)]

use wasm_bindgen::prelude::*;

pub mod console;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(module = "https://deno.land/x/denops_std@v0.3/mod.ts")]
extern "C" {
    pub type Vim;
    pub type VariableHelper;
    // static vim: Vim;

    #[wasm_bindgen(method, getter)]
    pub fn g(this: &Vim) -> VariableHelper;

    #[wasm_bindgen(method, getter, catch)]
    pub async fn get(this: &VariableHelper, prop: &str) -> Result<JsValue, JsValue>;
}

#[wasm_bindgen(start)]
pub fn initialize() -> Result<(), JsValue> {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    Ok(())
}

#[wasm_bindgen]
pub async fn greet2() -> JsValue {
    JsValue::from_str(&format!("{}", "Hello, denops-rust!"))
}

#[wasm_bindgen]
pub async fn vim_test(test: Vim) -> Vim {
    // console::log(JsValue::from(&test));
    test
}
