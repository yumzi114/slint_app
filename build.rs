fn main() {
    // println!("cargo:rerun-if-changed=ui/appwindow.slint");
    slint_build::compile("ui/app-window.slint").expect("Slint build failed");
}
