extern crate autotools;
extern crate bindgen;

use std::env;
use std::path::PathBuf;

use autotools::Config;

fn main() {
    let dst = Config::new("tre")
        .reconf("-ivf")
        .insource(true)
        .enable("static", None)
        .disable("shared", None)
        .build();

    println!("cargo:rustc-link-search=native={}", dst.join("lib").display());
    println!("cargo:rustc-link-lib=static=tre");
    println!("cargo:rustc-link-lib=c");

    let bindings = bindgen::Builder::default()
        .header("tre/include/tre.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
