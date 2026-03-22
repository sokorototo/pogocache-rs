// TODO: Pull latest release from: https://github.com/tidwall/pogocache/releases

fn main() {
    println!("cargo::rerun-if-changed=lib/pogocache.c");
    println!("cargo::rerun-if-changed=lib/pogocache.h");

    let build_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let lib_dir =
        std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("lib");

    // Generate glue code
    let bindings = bindgen::Builder::default()
        .header(lib_dir.join("pogocache.h").display().to_string())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(build_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!(
        "Generated bindings: {}",
        build_dir.join("bindings.rs").display()
    );

    // compile and link against pogocache.c
    cc::Build::new()
        .file(lib_dir.join("pogocache.c"))
        .flag("-O3")
        .out_dir(&build_dir)
        .compile("pogocache");

    println!("cargo:rustc-link-search={}", build_dir.display());
}
