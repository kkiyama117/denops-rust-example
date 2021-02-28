// Import 'start' function from denops_std
import { start } from "https://deno.land/x/denops_std@v0.3/mod.ts";
import * as pkg from '../../pkg/index.js'

// Call 'start' with async callback. The callback get 'vim' instance.
start(async (vim) => {
    // Initialize (loading wasm)
    await pkg.default();

    vim.register({
        async echo(_: unknown): Promise<unknown> {
            // async echo(text: unknown): Promise<unknown> {
            const text=pkg.greet();

            if (typeof text !== "string") {
                throw new Error(`'text' in 'echo()' of ${vim.name} must be a string`);
            }
            console.log(text);
            return await Promise.resolve(text);
        },
    });

    // Use 'vim.execute()' to execute Vim script
    await vim.execute(`
    command! DenopsRustEcho echo denops#request("${vim.name}", "echo", ["This is hello world message"])
  `);

    console.log("denops-rust.vim (std) has loaded");
});