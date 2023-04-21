fn main() {
    pkg_config::Config::new();
    let src = ["c/poly.c", "c/poly_ntt.c"];

    let mut builder = cc::Build::new();
    let build = builder
        .files(src.iter())
        .include("c")
        .flag("-Wno-unused-parameter")
        .flag("-mavx2")
        .flag("-o3")
        .define("USE_ZLIB", None);

    build.compile("cpoly");
}
