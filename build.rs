// TODO: Pull latest release from: https://github.com/tidwall/pogocache/releases

fn main() {
    println!("cargo::rerun-if-changed=lib/pogocache.c");
    println!("cargo::rerun-if-changed=lib/pogocache.h");

    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search=lib");

    let bindings = bindgen::Builder::default()
        .header("lib/pogocache.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("Generated bindings...");

    cc::Build::new()
        .file("lib/pogocache.c")
        .include("lib")
        // .flag("/std:c11")
        // .flag("/experimental:c11atomics")
        .compile("pogocache");
}
