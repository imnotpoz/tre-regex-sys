extern crate autotools;
extern crate bindgen;
extern crate fs_extra;

use std::env;
use std::path::PathBuf;
use std::process::Command;

use autotools::Config;

use fs_extra::dir::{copy, remove, CopyOptions};

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let tre_path = out_path.join("tre");

    // Clean up if we have to
    remove(&tre_path).ok();

    // Sigh, so tre does weird build stuff and we can't "just" run configure.
    // We have to run the generation script, but we aren't supposed to modify outside OUT_DIR, so,
    // we just copy it to out dir. Disgusting, I know. If you have a better option, please let me
    // know.
    let options = CopyOptions::new();
    copy("tre", &out_path, &options).expect("Failed to copy tre!");

    // Generate the config script.
    // I hate autotools so much.
    let script_path = tre_path.join("utils").join("autogen.sh");
    let script_path = script_path.to_str().unwrap();
    Command::new("sh")
        .current_dir(&tre_path)
        .args(["-c", script_path])
        .status()
        .expect("Could not run autogen.sh! Is autotools installed?");

    // Now do the actual configure/build stuff
    let dst = Config::new(out_path.join("tre"))
        .enable("static", None)
        .disable("shared", None)
        .disable("agrep", None)
        .build();

    // Clean up our mess
    remove(tre_path).expect("Could not clean up tre dir!");

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=tre");
    println!("cargo:rustc-link-lib=c");

    let include_path = out_path.join("include");
    let bindings = bindgen::Builder::default()
        .header(include_path.join("tre.h").to_str().unwrap())
        .newtype_enum("reg_errcode_t")
        .allowlist_function("tre_.*")
        .allowlist_type("(reg.*_t|tre_.*)")
        .allowlist_var("REG_.*")
        .blocklist_type("register_t")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
