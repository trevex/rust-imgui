extern crate gcc;

fn main() {
    gcc::Config::new()
        .cpp(true)
        .file("imgui-sys/imgui.cpp")
        .file("imgui-sys/imgui_render.cpp")
        .compile("libimgui.a");
}
