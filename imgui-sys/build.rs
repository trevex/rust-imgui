#![feature(path)]
#![feature(os)]

extern crate gcc;


fn main() {
    println!("cargo:rustc-flags=-l dylib=stdc++");

    let root = Path::new(std::os::getenv("CARGO_MANIFEST_DIR").unwrap())
        .join("imgui-sys");

    let config = gcc::Config {
        include_directories: vec![
            root.clone(),
        ],
        definitions: vec![],
        objects: vec![],
        flags: vec![],
    };

    println!("cargo:include={}", root.join("include").display());

    gcc::compile_library("libimgui.a", &config, &[
        "imgui-sys/imgui.cpp",
        "imgui-sys/imgui_render.cpp"
    ]);
}
