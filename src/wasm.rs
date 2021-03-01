use wasm_bindgen::prelude::*;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[macro_export]
macro_rules! console_log {
    ($($arg:tt)*) => {
         $crate::wasm::log(&format!($($arg)*))
    };
}

#[macro_export]
macro_rules! console_error {
    ($($arg:tt)*) => {
         $crate::wasm::log_error(&format!($($arg)*))
    };
}

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = error)]
    fn log_error(s: &str);
    #[wasm_bindgen(js_namespace = global, js_name = sleep, catch)]
    async fn deno_sleep(sec: u64) -> Result<(), JsValue>;
}

#[wasm_bindgen]
pub fn greet() -> JsValue {
    console_log!("Hello, denops, from rust!");
    console_error!("Hello, denops, from rust!");
    JsValue::from_str(&format!("{}", "Hello, denops-rust!"))
}

#[wasm_bindgen]
pub async fn greet2() -> JsValue {
    console_log!("sleepy ...");
    deno_sleep(5).await;
    console_log!("Timer!");
    JsValue::from_str(&format!("{}", "Hello, denops-rust!"))
}
