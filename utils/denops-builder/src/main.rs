use duct::cmd;
use duct_sh::sh;
use std::{env, path::PathBuf, process::Command};

fn main() {
    // use getopts::Options;
    // let args: Vec<String> = env::args().collect();
    // let output_dir = matches.opt_str("o").unwrap();
    let path = env::current_dir().unwrap();
    let out_file = "index";
    denops_build(&path, &out_file);
}

fn denops_build(base_dir: &PathBuf, out_file: &str) -> anyhow::Result<()> {
    cmd!(
        "wasm-pack",
        "build",
        "--target",
        "web",
        "--out-name",
        out_file
    )
    .read()?;
    cmd!(
        "sed",
        "-i",
        "-e",
        "s#input = fetch(input);#if (typeof Deno !== 'undefined') input = new WebAssembly.Module(await Deno.readFile(new URL(input).pathname));#",
        &base_dir.join("pkg").join(out_file.to_owned() + ".js"))
    .read()?;
    Ok(())
}
