extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    cc::Build::new()
        .cpp(true)
        .opt_level(3)
        .file("clipper/clipper.cpp")
        .file("clipper/wrapper.cpp")
        .compile("clipper");

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap();

    match (target_os.as_str(), target_env.as_str()) {
        ("linux", _) | ("windows", "gnu") => println!("cargo:rustc-link-lib=dylib=stdc++"),
        ("macos", _) => println!("cargo:rustc-link-lib=dylib=c++"),
        ("windows", "msvc") => {}
        _ => unimplemented!(
            "target_os: {}, target_env: {}",
            target_os.as_str(),
            target_env.as_str()
        ),
    }

    let bindings = bindgen::Builder::default()
        .header("clipper/wrapper.h")
        .whitelist_type("Polygons")
        .whitelist_type("ClipType")
        .whitelist_type("JoinType")
        .whitelist_type("EndType")
        .whitelist_type("PolyType")
        .whitelist_type("PolyFillType")
        .whitelist_type("Vertice")
        .whitelist_type("Path")
        .whitelist_type("Polygon")
        .whitelist_function("clean")
        .whitelist_function("simplify")
        .whitelist_function("execute")
        .whitelist_function("offset")
        .whitelist_function("free_path")
        .whitelist_function("free_polygon")
        .whitelist_function("free_polygons")
        .generate()
        .expect("unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("couldn't write bindings!");
}
