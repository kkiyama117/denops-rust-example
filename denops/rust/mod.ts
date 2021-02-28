// Import 'start' function from denops_std
import { start } from "https://deno.land/x/denops_std@v0.3/mod.ts";
import * as pkg from '../../pkg/index.js'

// Initialize (loading wasm)
await pkg.default();

// Call 'start' with async callback. The callback get 'vim' instance.
start(async (vim) => {
    // Register RPC functions with 'vim.register' like:
    vim.register({
        // Developers can define multiple endpoints which take arbitrary number of arguments
        // and return arbitrary value as a Promise.
        // This function can be called by denops#request() or denops#notify() functions.
        async echo(text: unknown): Promise<unknown> {
            text=pkg.greet();

            if (typeof text !== "string") {
                throw new Error(`'text' in 'echo()' of ${vim.name} must be a string`);
            }
            // console.log (console.info, console.debug as well) output message to Vim echo
            // area with [denops] prefix.
            console.log("echo is called");

            return await Promise.resolve(text);
        },
        async get_variables(): Promise<void> {
            // Use 'vim.g.get' to access global variable
            console.log("g:denops_helloworld", await vim.g.get("denops_helloworld"));
            // Use 'vim.b.get' to access buffer-local variable
            console.log("b:denops_helloworld", await vim.b.get("denops_helloworld"));
            // Use 'vim.w.get' to access window-local variable
            console.log("w:denops_helloworld", await vim.w.get("denops_helloworld"));
            // Use 'vim.t.get' to access tabpage-local variable
            console.log("t:denops_helloworld", await vim.t.get("denops_helloworld"));
            // Use 'vim.v.get' to access Vim's variable
            console.log("v:errmsg", await vim.v.get("errmsg"));
            // Use 'vim.env.get' to access environment variable
            console.log("$DENOPS_HELLOWORLD", await vim.env.get("DENOPS_HELLOWORLD"));
        },

        async set_variables(): Promise<void> {
            // Use 'vim.g.set' to replace global variable
            await vim.g.set("denops_helloworld", "Global HOGEHOGE");
            // Use 'vim.b.set' to replace buffer-local variable
            await vim.b.set("denops_helloworld", "Buffer HOGEHOGE");
            // Use 'vim.w.set' to replace window-local variable
            await vim.w.set("denops_helloworld", "Window HOGEHOGE");
            // Use 'vim.t.set' to replace tabpage-local variable
            await vim.t.set("denops_helloworld", "Tabpage HOGEHOGE");
            // Use 'vim.v.set' to replace Vim's variable
            await vim.v.set("errmsg", "Vim HOGEHOGE");
            // Use 'vim.env.set' to replace environment variable
            await vim.env.set("DENOPS_HELLOWORLD", "Env HOGEHOGE");
        },

        async remove_variables(): Promise<void> {
            // Use 'vim.g.remove' to remove global variable
            await vim.g.remove("denops_helloworld");
            // Use 'vim.b.remove' to remove buffer-local variable
            await vim.b.remove("denops_helloworld");
            // Use 'vim.w.remove' to remove window-local variable
            await vim.w.remove("denops_helloworld");
            // Use 'vim.t.remove' to remove tabpage-local variable
            await vim.t.remove("denops_helloworld");
            // Use 'vim.v.remove' to remove Vim variable
            await vim.v.remove("errmsg");
            // Use 'vim.env.remove' to remove environment variable
            await vim.env.remove("DENOPS_HELLOWORLD");
        },

        async register_autocmd(): Promise<void> {
            await vim.cmd("new");
            // Use 'vim.autocmd' to register autocmd
            await vim.autocmd("denops_rust", (helper) => {
                // Use 'helper.remove()' to remove autocmd
                helper.remove("*", "<buffer>");
                // Use 'helper.define()' to define autocmd
                helper.define(
                    "CursorHold",
                    "<buffer>",
                    "echomsg 'Hello Denops CursorHold'",
                );
                helper.define(
                    ["BufEnter", "BufLeave"],
                    "<buffer>",
                    "echomsg 'Hello Denops BufEnter/BufLeave'",
                );
            });
        },
    });

    // Use 'vim.execute()' to execute Vim script
    await vim.execute(`
    command! DenopsRustEcho echo denops#request("${vim.name}", "echo", ["This is hello world message"])
    command! DenopsRustGetVariables echo denops#notify("${vim.name}", "get_variables", [])
    command! DenopsRustSetVariables echo denops#notify("${vim.name}", "set_variables", [])
    command! DenopsRustRemoveVariables echo denops#notify("${vim.name}", "remove_variables", [])
    command! DenopsRustRegisterAutocmd echo denops#notify("${vim.name}", "register_autocmd", [])
    let g:denops_helloworld = "Global Hello Denops"
    let b:denops_helloworld = "Buffer Hello Denops"
    let w:denops_helloworld = "Window Hello Denops"
    let t:denops_helloworld = "Tabpage Hello Denops"
    let v:errmsg = "Vim Hello Denops"
    let $DENOPS_HELLOWORLD = "Env Hello Denops"
  `);

    console.log("denops-rust.vim (std) has loaded");
});