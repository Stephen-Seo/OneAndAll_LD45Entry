extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=raylib");
    println!("cargo:rustc-link-lib=OpenGL");
    println!("cargo:rerun-if-changed=raylib/raylib.h");

    let bindings = bindgen::Builder::default()
        .header("raylib/raylib.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .clang_arg("-fvisibility=default")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("raylib_bindings.rs"))
        .expect("Couldn't write bindings!");
}
