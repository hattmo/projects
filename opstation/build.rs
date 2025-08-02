use std::env;
use std::path::PathBuf;

use bindgen::{Builder, FieldVisibilityKind};

fn main() {
    cc::Build::new().file("wg/wireguard.c").compile("wg");
    let bindings = Builder::default()
        .header("wg/wireguard.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .default_visibility(FieldVisibilityKind::Private)
        .allowlist_item("wg_.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("wg_binding.rs"))
        .expect("Couldn't write bindings!");
}
