// Import 'start' function from denops_std
import { start } from "https://deno.land/x/denops_std@v0.3/mod.ts";
import { delay } from "https://deno.land/std/async/mod.ts";
import * as pkg from '../../pkg/index.js'

// Call 'start' with async callback. The callback get 'vim' instance.
start(async (vim) => {
    // Initialize (loading wasm)
    await pkg.default();
    pkg.initialize();

    vim.register({
        // async echo(_: unknown): Promise<unknown> {
        async echo(_: unknown): Promise<void> {
            const test= await pkg.vim_test(vim);
            console.log("g:denops_helloworld", await vim.g.get("denops_helloworld"));
        },
    });

    // Use 'vim.execute()' to execute Vim script
    await vim.execute(`
      command! DenopsRustEcho echo denops#request("${vim.name}", "echo", [""])
      let g:denops_helloworld = "Global Hello Denops"
  `);

    console.log("denops-rust.vim has loaded");
});