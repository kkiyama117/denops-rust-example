/*!

# Denops Rust Example

## About

Read this [article](https://zenn.dev/kkiyama117/articles/2021-03-01-denops-rust) also.

generated by [rustwasm/wasm-pack-template](https://github.com/rustwasm/wasm-pack-template)

## Todo

- [x] Build wasm from Rust and run on denops platform
- [ ] Add type and trait wrapping `denops-deno` and `denops-deno-std`

## 🛠️ Build

### Depends on

- sed command
- wasm-pack(you can get it by `cargo install wasm-pack`)

### Build

```bash
  # run wasm-pack and fix something to run by deno
  cargo run --manifest-path=utils/denops-builder/Cargo.toml
```

## LICENSE

This program is dual licensed by MIT and apache because of its dependencies.
See each licence also. ([`LICENSE_MIT`](https://github.com/kkiyama117/denops-rust-example/blob/main/LICENSE_MIT) and [`LICENSE_APACHE`](https://github.com/kkiyama117/denops-rust-example/blob/main/LICENSE_APACHE))

## Relations

- https://github.com/vim-denops/denops-deno
- https://github.com/vim-denops/denops-std-deno

## Info

- https://github.com/rustwasm/wasm-pack/issues/672

*/
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{JsFuture, future_to_promise};
use web_sys::{Response};
use denops_rust::{console_log, Vim};
use js_sys::Promise;

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
pub async fn vim_test(vim: Vim) -> JsValue {
    // Use `console::log` (defined at denops_rust) to call `echo` in code written by rust!
    console_log!("You can get variables and return it!");
    // You can use (return) JsValue from function!
    vim.g().get("denops_helloworld").await.unwrap()
}

#[wasm_bindgen]
pub async fn vim_test2(_vim: Vim) {
    // Can call and wait deno's asynchronous function here!
    let data = call_js_fetch("https://api.github.com/repos/kkiyama117/denops-rust-example").await.unwrap();
    console_log!("{:?}",data);
    console_log!("Call asynchronous function finished!");
}

pub async fn _vim_test3(_vim: Vim) -> Result<JsValue,JsValue> {
    // Can call and wait deno's asynchronous function here!
    let data = call_js_fetch("https://api.github.com/repos/kkiyama117/denops-rust-example").await?;
    console_log!("{:?}",data);
    Ok(JsValue::from_str("fetch finished"))
}

#[wasm_bindgen]
pub async fn vim_test3(_vim: Vim) -> Promise {
    future_to_promise(_vim_test3(_vim))
}

#[wasm_bindgen]
extern "C" {
    // Import `fetch` API of deno
    #[wasm_bindgen(js_name = fetch, catch)]
    async fn js_fetch(s: &str) -> Result<JsValue, JsValue>;
}

// demo async function to call bind func
async fn call_js_fetch(url: &str) -> Result<JsValue, JsValue> {
    let resp_value = js_fetch(url).await?;
    let resp: Response = resp_value.into();
    let json = JsFuture::from(resp.json()?).await?;

    Ok(json)
}
