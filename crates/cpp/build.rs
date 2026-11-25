fn main() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let cpp = root.join("../../cpp");

    cc::Build::new()
        .cpp(true)
        .file(cpp.join("src/nodule.cpp"))
        .include(cpp.join("include"))
        .flag_if_supported("-std=c++17")
        .compile("nodule_cpp");

    println!("cargo:rerun-if-changed={}", cpp.join("src").display());
    println!("cargo:rerun-if-changed={}", cpp.join("include").display());
}