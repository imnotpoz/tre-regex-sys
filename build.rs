extern crate autotools;
extern crate bindgen;

use std::env;
use std::path::PathBuf;

use autotools::Config;

fn main() {
    let dst = Config::new("tre")
        .reconf("-ivf")
        .enable("static", None)
        .disable("shared", None)
        .disable("agrep", None)
        .build();

    println!("cargo:rustc-link-search=native={}", dst.join("lib").display());
    println!("cargo:rustc-link-lib=static=tre");
    println!("cargo:rustc-link-lib=c");
    
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
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
