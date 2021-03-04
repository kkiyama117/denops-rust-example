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
    let result =
        Command::new("wasm-pack")
            .args(&["build", "--target", "web", "--out-name", out_file])
            .output()
            .expect("failed to build by wasm-pack");
    let result = String::from_utf8(result.stderr).unwrap();
    println!("{}", result);
    let result2 =
    Command::new("sed")
        .args(&["-i", "-e", "s#input = fetch(input);#if (typeof Deno !== 'undefined') input = new WebAssembly.Module(await Deno.readFile(new URL(input).pathname));#", ])
        .arg(&base_dir.join("pkg").join(out_file.to_owned() + ".js"))
        .output()
        .expect("failed to run sed");
    let result2 = String::from_utf8(result2.stderr).unwrap();
    println!("{}", result2);
}
