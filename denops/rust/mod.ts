// Import 'start' function from denops_std
import {start} from "https://deno.land/x/denops_std@v0.3/mod.ts";
import * as pkg from '../../pkg/index.js'

// Call 'start' with async callback. The callback get 'vim' instance.
start(async (vim) => {
    // Initialize (loading wasm)
    await pkg.default();
    pkg.initialize();

    vim.register({
        async test1(_: unknown): Promise<void> {
            const test = await pkg.vim_test(vim);
            console.log(test);
        },
        async test2(_: unknown): Promise<void> {
            await pkg.vim_test2(vim);
        },
    });

    // Use 'vim.execute()' to execute Vim script
    await vim.execute(`
      command! DenopsRustTest1 echo denops#request("${vim.name}", "test1", [""])
      command! DenopsRustTest2 echo denops#request("${vim.name}", "test2", [""])
      let g:denops_helloworld = "Global Hello Denops"
  `);

    console.log("denops-rust-example.vim has loaded");
});