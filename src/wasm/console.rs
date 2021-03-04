use wasm_bindgen::prelude::*;

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
#[no_mangle]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: JsValue);
    #[wasm_bindgen(js_namespace = console, js_name = error)]
    pub fn log_error(s: JsValue);
    // #[wasm_bindgen(js_namespace = global, js_name = sleep, catch)]
    // #[wasm_bindgen(js_name = sleep, catch)]
    // async fn sleep(m_sec: JsValue) -> Result<(), JsValue>;
}
