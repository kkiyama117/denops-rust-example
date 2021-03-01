use std::{env, path::PathBuf, process::Command};

fn main() {
    // use getopts::Options;
    // let args: Vec<String> = env::args().collect();
    // let output_dir = matches.opt_str("o").unwrap();
    let path = env::current_dir().unwrap();
    let out_file = "index";
    denops_build(&path, &out_file);
}

fn denops_build(base_dir: &PathBuf, out_file: &str) {
    Command::new("wasm-pack")
        .args(&["build", "--target", "web", "--out-name", out_file])
        .output()
        .ok();
    let result = Command::new("sed")
        .args(&["-i", "-e", "s#input = fetch(input);#if (typeof Deno !== 'undefined') input = new WebAssembly.Module(await Deno.readFile(new URL(input).pathname));#",])
        .arg( &base_dir.join("pkg").join(out_file.to_owned()+".js"))
        .output()
        .expect("failed to run sed");
    println!("{:?}", result);
}
