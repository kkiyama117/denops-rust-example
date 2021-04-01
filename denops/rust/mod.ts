// Import 'start' function from denops_std
import {start} from "https://deno.land/x/denops_std@v0.4/mod.ts";
import * as pkg from '../../pkg/index.js'

// Call 'start' with async callback. The callback get 'vim' instance.
start(async (vim) => {
    // Initialize (loading wasm)
    await pkg.default();
    pkg.initialize();

    vim.register({
        async test1(_: unknown): Promise<unknown> {
            const test = await pkg.vim_test(vim);
            console.log(test);
        },
        async test2(_: unknown): Promise<unknown> {
            await pkg.vim_test2(vim);
        },

        async test3(_: unknown): Promise<unknown> {
            await pkg.vim_test3(vim);
        },
    });

    // Use 'vim.execute()' to execute Vim script
    await vim.execute(`
      command! DenopsRustTest1 echo denops#request("${vim.name}", "test1", [""])
      command! DenopsRustTest2 echo denops#request("${vim.name}", "test2", [""])
      command! DenopsRustTest3 echo denops#request("${vim.name}", "test3", [""])
      let g:denops_helloworld = "Global Hello Denops"
  `);

    console.log("denops-rust-example.vim has loaded");
});