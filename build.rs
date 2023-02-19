extern crate bindgen;

use std::env;
use std::path::PathBuf;

#[cfg(not(feature = "no_link_raylib"))]
fn linking_raylib() {
    println!("cargo:rustc-link-lib=raylib");
}

#[cfg(feature = "no_link_raylib")]
fn linking_raylib() {}

fn main() {
    linking_raylib();
    println!("cargo:rerun-if-changed=raylib/raylib.h");

    let bindings = bindgen::Builder::default()
        .header("raylib/raylib.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("raylib_bindings.rs"))
        .expect("Couldn't write bindings!");
}
