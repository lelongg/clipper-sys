use bindgen;
use std::io::stdout;

fn main() {
    let bindings = bindgen::Builder::default()
        .header("../clipper/wrapper.h")
        .whitelist_type("Polygons")
        .whitelist_type("ClipType")
        .whitelist_type("JoinType")
        .whitelist_type("EndType")
        .whitelist_type("PolyType")
        .whitelist_type("PolyFillType")
        .whitelist_type("Vertice")
        .whitelist_type("Path")
        .whitelist_type("Polygon")
        .whitelist_function("execute")
        .whitelist_function("offset")
        .whitelist_function("free_path")
        .whitelist_function("free_polygon")
        .whitelist_function("free_polygons")
        .generate()
        .expect("unable to generate bindings");

    bindings
        .write(Box::new(stdout()))
        .expect("Writing bindings to stdout");
}
