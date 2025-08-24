use std::{env, path::PathBuf};

use bindgen::Builder;

fn main() {
    cc::Build::new().file("extern/wireguard.c").compile("wg");
    println!("cargo:rustc-link-lib=nl-route-3");
    println!("cargo:rustc-link-lib=nl-3");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    Builder::default()
        .header("extern/wireguard.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .allowlist_item("wg_.*")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("wg_binding.rs"))
        .expect("Couldn't write bindings!");

    Builder::default()
        .header("extern/nl.h")
        .clang_arg("-I/usr/include/libnl3")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .allowlist_item("nl_.*")
        .allowlist_item("rtnl_.*")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("nl_binding.rs"))
        .expect("Couldn't write bindings!");
}
